import { chromium } from '@playwright/test';
import { spawn, execSync } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const SERVER_URL = 'http://127.0.0.1:50051';
const CLIENT_URL = 'http://127.0.0.1:8081';

let serverProc = null;
let httpServerProc = null;

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function startServer() {
  return new Promise((resolve, reject) => {
    const cargoPath = process.env.CARGO_PATH || '/home/ubuntu/.cargo/bin/cargo';
    serverProc = spawn(cargoPath, ['run', '-p', 'example-server'], {
      cwd: join(__dirname, '../..'),
    });

    serverProc.stdout.on('data', (data) => {
      const str = data.toString();
      console.log(`[server] ${str}`);
      if (str.includes('Greeter server listening')) {
        resolve();
      }
    });

    serverProc.stderr.on('data', (data) => {
      console.error(`[server error] ${data}`);
    });

    serverProc.on('error', reject);
    
    setTimeout(() => resolve(), 10000);
  });
}

async function startHttpServer() {
  const distPath = join(__dirname, '../../../dist');
  console.log(`Starting HTTP server from: ${distPath}`);
  
  const pythonPath = process.env.PYTHON_PATH || '/usr/bin/python3';
  
  return new Promise((resolve, reject) => {
    httpServerProc = spawn(pythonPath, ['-m', 'http.server', '8081', '-d', distPath]);

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
    
    const consoleMessages = [];
    page.on('console', msg => {
      const text = msg.text();
      consoleMessages.push(text);
      console.log(`[browser console] [${msg.type()}] ${text}`);
    });
    page.on('pageerror', error => console.log(`[browser pageerror] ${error}`));
    page.on('requestfailed', request => console.log(`[browser requestfailed] ${request.url()} - ${request.failure().errorText}`));
    page.on('response', response => {
      if (!response.ok()) {
        console.log(`[browser response] ${response.url()} - ${response.status()}`);
      }
    });
    
    console.log(`Navigating to ${CLIENT_URL}...`);
    await page.goto(CLIENT_URL, { waitUntil: 'networkidle0', timeout: 30000 });
    
    const title = await page.title();
    console.log(`Page title: ${title}`);
    
    // Wait for WASM to initialize
    await sleep(2000);
    
    // Check what's on the page
    const bodyContent = await page.content();
    console.log(`Page has content: ${bodyContent.length} chars`);
    
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
    console.log('Input field found');
    
    await page.fill('input[type="text"]', 'TestUser');
    console.log('Entered text in input');
    
    await page.click('button');
    console.log('Clicked submit button');
    
    await sleep(2000);
    
    const responseText = await page.locator('p').first().textContent();
    console.log(`Response text: ${responseText}`);
    
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
  
  if (serverProc) {
    serverProc.kill();
    console.log('Server stopped');
  }
  
  if (httpServerProc) {
    httpServerProc.kill();
    console.log('HTTP server stopped');
  }
}

async function main() {
  try {
    console.log('Starting Rust server...');
    await startServer();
    
    console.log('Starting HTTP server for client...');
    await startHttpServer();
    
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
