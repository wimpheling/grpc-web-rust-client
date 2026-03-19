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

    console.log(`Navigating to ${CLIENT_URL}/count...`);
    await page.goto(`${CLIENT_URL}/count`, { waitUntil: 'networkidle0', timeout: 30000 });

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

    if (heading !== 'Count Arithmetic Progression') {
      throw new Error(`Expected heading "Count Arithmetic Progression", got "${heading}"`);
    }

    console.log('✓ Test passed: Page loaded with correct heading');

    // Test input and button
    await page.waitForSelector('input[type="number"]', { timeout: 5000 });

    // Clear input and set to 1 (default)
    await page.fill('input[type="number"]', '1');

    await page.click('button');

    // Wait for streaming messages to appear
    let logText = '';
    for (let i = 0; i < 30; i++) {
      await sleep(500);
      const logDiv = await page.locator('#stream-log').textContent();
      if (logDiv && logDiv.includes('Received:')) {
        logText = logDiv;
        console.log(`Found streaming log: ${logText}`);
        break;
      }
    }

    // Verify the progression 1, 2, 4, 8, 16
    const expectedValues = ['1', '2', '4', '8', '16'];
    const allFound = expectedValues.every(val => logText.includes(`Received: ${val}`));

    if (!allFound) {
      throw new Error(`Expected to find all values 1, 2, 4, 8, 16 in log, got: ${logText}`);
    }

    console.log('✓ Test passed: Streaming arithmetic progression works correctly');
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
