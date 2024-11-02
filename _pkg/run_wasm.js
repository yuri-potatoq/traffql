const runtime = chrome.runtime || browser.runtime;

(
async () => {
    return await wasm_bindgen(runtime.getURL('traffiQL_bg.wasm'));
})()
