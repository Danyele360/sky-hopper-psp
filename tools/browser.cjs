// Use the bundled Playwright browser, or an explicitly selected local browser.
const {chromium} = require('playwright');
const fs = require('node:fs');
async function launchBrowser() {
  const explicit = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE;
  const windowsChrome = 'C:/Program Files/Google/Chrome/Application/chrome.exe';
  const executablePath = explicit || (process.platform === 'win32' && fs.existsSync(windowsChrome) ? windowsChrome : undefined);
  return chromium.launch({headless: true, ...(executablePath ? {executablePath} : {})});
}
module.exports = {launchBrowser};
