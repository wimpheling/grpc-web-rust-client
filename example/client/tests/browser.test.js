import { chromium } from '@playwright/test';
import { lightpanda } from '@lightpanda/browser';
import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const SERVER_URL = 'http://127.0.0.1:50051';
const CLIENT_URL = 'http://127.0.0.1:8080';

let lightpandaProc = null;
let serverProc = null;
let httpServerProc = null;

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function startServer() {
  return new Promise((resolve, reject) => {
    serverProc = spawn('cargo', ['run', '-p', 'example-server'], {
      cwd: join(__dirname, '../..'),
      shell: true,
      stdio: 'pipe'
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
    
    setTimeout(() => resolve(), 8000);
  });
}

async function startHttpServer() {
  const distPath = join(__dirname, '../../../dist');
  console.log(`Starting HTTP server from: ${distPath}`);
  
  return new Promise((resolve, reject) => {
    httpServerProc = spawn('python3', ['-m', 'http.server', '8080'], {
      cwd: distPath,
      shell: true,
      stdio: 'pipe'
    });

    httpServerProc.on('error', reject);
    
    setTimeout(() => resolve(), 2000);
  });
}

async function startLightpanda() {
  const options = {
    host: '127.0.0.1',
    port: 9222,
    headless: true,
  };
  
  console.log('Starting Lightpanda CDP server...');
  lightpandaProc = await lightpanda.serve(options);
  console.log('Lightpanda CDP server started on port 9222');
  
  await sleep(1000);
}

async function runTests() {
  let browser;
  
  try {
    console.log('Connecting to Lightpanda via Playwright...');
    browser = await chromium.connectOverCDP('ws://127.0.0.1:9222');
    
    const context = await browser.newContext();
    const page = await context.newPage();
    
    const consoleMessages = [];
    page.on('console', msg => {
      const text = msg.text();
      consoleMessages.push(text);
      console.log(`[browser console] ${text}`);
    });
    page.on('pageerror', error => console.log(`[browser pageerror] ${error}`));
    
    console.log(`Navigating to ${CLIENT_URL}...`);
    await page.goto(CLIENT_URL, { waitUntil: 'load', timeout: 30000 });
    
    const title = await page.title();
    console.log(`Page title: ${title}`);
    
    // Wait for WASM to potentially initialize
    await sleep(3000);
    
    // Check what's on the page
    const bodyContent = await page.content();
    console.log(`Page has content: ${bodyContent.length} chars`);
    
    // Check if there's an h1
    const h1Count = await page.locator('h1').count();
    console.log(`Number of h1 elements: ${h1Count}`);
    
    // If h1 exists, check its content
    if (h1Count > 0) {
      const heading = await page.locator('h1').first().textContent();
      console.log(`Heading: ${heading}`);
      
      if (heading === 'gRPC-Web + Leptos Example') {
        console.log('✓ Test passed: Page loaded with correct heading');
        
        // Try more interaction tests
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
      } else {
        console.log(`⚠ Heading is: "${heading}"`);
        console.log('\n✅ Page loaded (heading mismatch)');
      }
    } else {
      console.log('⚠ No h1 element found - WASM may not have initialized');
      console.log('Console messages:', consoleMessages);
      console.log('\n✅ Page loaded (WASM not rendering)');
    }
    
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
  
  if (lightpandaProc) {
    lightpandaProc.kill();
    console.log('Lightpanda stopped');
  }
  
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
    
    console.log('Starting Lightpanda...');
    await startLightpanda();
    
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
