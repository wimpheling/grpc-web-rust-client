import { chromium } from '@playwright/test';

const CLIENT_URL = 'http://127.0.0.1:8082';

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function main() {
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

main();
