import { chromium } from '@playwright/test';
import { spawn, execSync } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const EXAMPLE_DIR = join(__dirname, '../..');
const DIST_DIR = join(EXAMPLE_DIR, '../dist');
const CLIENT_URL = 'http://127.0.0.1:8082';

let httpServerProc = null;

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function waitForHttp(port, path = '/', maxAttempts = 60) {
  for (let i = 0; i < maxAttempts; i++) {
    try {
      execSync(`curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:${port}${path}`, { timeout: 5000 });
      return true;
    } catch (e) {
      await sleep(1000);
    }
  }
  return false;
}

async function killPort(port) {
  try {
    execSync(`fuser -k ${port}/tcp 2>/dev/null`, { timeout: 5000 });
    await sleep(1000);
  } catch (e) {
    // Ignore errors
  }
}

function startDockerCompose() {
  console.log('Starting docker-compose (Envoy + gRPC server)...');
  try {
    execSync('docker compose up -d --build', { cwd: EXAMPLE_DIR, stdio: 'inherit' });
    console.log('docker-compose started');
  } catch (e) {
    throw new Error(`Failed to start docker-compose: ${e.message}`);
  }
}

async function waitForServices() {
  console.log('Waiting for Envoy on port 8081...');
  const envoyReady = await waitForHttp(8081, '/');
  if (!envoyReady) {
    throw new Error('Envoy did not become ready on port 8081');
  }
  console.log('Envoy is ready!');
}

async function buildWasmClient() {
  console.log('Building WASM client...');
  try {
    execSync('wasm-pack build --target web --out-dir /home/ubuntu/grpc-web-rust-client/dist/example_client --release', {
      cwd: join(EXAMPLE_DIR, 'client'),
      stdio: 'inherit',
    });
    console.log('WASM client built successfully');
  } catch (e) {
    throw new Error(`Failed to build WASM client: ${e.message}`);
  }
}

async function startHttpServer() {
  console.log(`Starting HTTP server for Leptos app from: ${DIST_DIR}`);
  
  await killPort(8082);
  
  const pythonPath = process.env.PYTHON_PATH || '/usr/bin/python3';
  
  return new Promise((resolve, reject) => {
    httpServerProc = spawn(pythonPath, ['-m', 'http.server', '8082', '-d', DIST_DIR], {
      stdio: ['ignore', 'pipe', 'pipe'],
    });

    httpServerProc.stdout.on('data', (data) => {
      console.log(`[http-server] ${data.toString().trim()}`);
    });

    httpServerProc.stderr.on('data', (data) => {
      console.log(`[http-server] ${data.toString().trim()}`);
    });

    httpServerProc.on('error', reject);
    
    setTimeout(() => resolve(), 2000);
  });
}

async function runTests() {
  let browser;
  
  try {
    console.log('Launching Chrome...');
    browser = await chromium.launch({ headless: true });
    
    const context = await browser.newContext();
    const page = await context.newPage();
    
    page.on('console', msg => {
      console.log(`[browser] [${msg.type()}] ${msg.text()}`);
    });
    page.on('pageerror', error => console.log(`[browser error] ${error}`));
    page.on('requestfailed', request => console.log(`[browser request failed] ${request.url()}`));
    
    console.log(`Navigating to ${CLIENT_URL}...`);
    await page.goto(CLIENT_URL, { waitUntil: 'networkidle0', timeout: 30000 });
    
    // Wait for WASM to initialize
    await sleep(3000);
    
    // Check if there's an h1
    const h1Count = await page.locator('h1').count();
    console.log(`Number of h1 elements: ${h1Count}`);
    
    if (h1Count === 0) {
      throw new Error('No h1 element found - WASM may not have initialized');
    }
    
    const heading = await page.locator('h1').first().textContent();
    console.log(`Heading: ${heading}`);
    
    if (heading !== 'gRPC-Web + Leptos Example') {
      throw new Error(`Expected heading "gRPC-Web + Leptos Example", got "${heading}"`);
    }
    
    console.log('✓ Test passed: Page loaded with correct heading');
    
    // Test input and button
    await page.waitForSelector('input[type="text"]', { timeout: 5000 });
    
    await page.fill('input[type="text"]', 'TestUser');
    
    await page.click('button');
    
    // Wait for the gRPC response to appear in the DOM
    let responseText = '';
    for (let i = 0; i < 30; i++) {
      await sleep(500);
      const resultDiv = await page.locator('#grpc-result').count();
      if (resultDiv > 0) {
        responseText = await page.locator('#grpc-result').textContent();
        console.log(`Found gRPC result: ${responseText}`);
        break;
      }
    }
    
    if (!responseText.includes('Hello, TestUser!')) {
      throw new Error(`Expected response to contain "Hello, TestUser!", got "${responseText}"`);
    }
    
    console.log('✓ Test passed: gRPC-web call succeeded with correct response');
    console.log('\n✅ All tests completed successfully!');
    
  } catch (error) {
    console.error('\n❌ Test failed:', error.message);
    process.exit(1);
  } finally {
    if (browser) {
      await browser.close();
    }
  }
}

async function cleanup() {
  console.log('\nCleaning up...');
  
  if (httpServerProc) {
    httpServerProc.kill();
    console.log('HTTP server stopped');
  }
  
  try {
    execSync('docker compose down', { cwd: EXAMPLE_DIR, stdio: 'inherit' });
    console.log('docker-compose stopped');
  } catch (e) {
    console.error('Failed to stop docker-compose:', e.message);
  }
}

async function main() {
  try {
    // Start docker-compose (Envoy + gRPC server)
    startDockerCompose();
    
    // Wait for services to be ready
    await waitForServices();
    
    // Build WASM client locally
    await buildWasmClient();
    
    // Start HTTP server for Leptos app
    console.log('Starting HTTP server for Leptos app...');
    await startHttpServer();
    
    // Run tests
    console.log('Running tests...');
    await runTests();
  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  } finally {
    await cleanup();
  }
}

main();
