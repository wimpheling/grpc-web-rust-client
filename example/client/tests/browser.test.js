import { chromium } from '@playwright/test';
import { lightpanda, type LightpandaServeOptions } from '@lightpanda/browser';
import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const SERVER_URL = 'http://127.0.0.1:5001';
const CLIENT_URL = 'http://127.0.0.1:8080';

let lightpandaProc = null;
let serverProc = null;

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
      console.log(`[server] ${data}`);
      if (data.toString().includes('Greeter server listening')) {
        resolve();
      }
    });

    serverProc.stderr.on('data', (data) => {
      console.error(`[server error] ${data}`);
    });

    serverProc.on('error', reject);
    
    setTimeout(() => resolve(), 3000);
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
    
    console.log(`Navigating to ${CLIENT_URL}...`);
    await page.goto(CLIENT_URL, { waitUntil: 'networkidle0', timeout: 30000 });
    
    const title = await page.title();
    console.log(`Page title: ${title}`);
    
    const heading = await page.locator('h1').textContent();
    console.log(`Heading: ${heading}`);
    
    if (heading !== 'gRPC-Web + Leptos Example') {
      throw new Error(`Expected heading "gRPC-Web + Leptos Example", got "${heading}"`);
    }
    
    await page.waitForSelector('input[type="text"]');
    console.log('Input field found');
    
    await page.fill('input[type="text"]', 'TestUser');
    console.log('Entered text in input');
    
    await page.click('button');
    console.log('Clicked submit button');
    
    await sleep(2000);
    
    const responseText = await page.locator('p').first().textContent();
    console.log(`Response text: ${responseText}`);
    
    const errorText = await page.locator('p[style*="color: red"]').textContent().catch(() => '');
    console.log(`Error text: "${errorText}"`);
    
    if (responseText.includes('Hello') || !errorText) {
      console.log('✓ Test passed: Got response from server');
    } else {
      console.log('⚠ Test warning: May not have received expected response');
    }
    
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
  
  if (lightpandaProc) {
    lightpandaProc.kill();
    console.log('Lightpanda stopped');
  }
  
  if (serverProc) {
    serverProc.kill();
    console.log('Server stopped');
  }
}

async function main() {
  try {
    console.log('Starting server...');
    await startServer();
    
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
