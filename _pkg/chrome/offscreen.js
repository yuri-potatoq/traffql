/**
 * Notes:
 *
 * Can't spawn sqlite instance using OPFS here because offscreen don't provide appropriated API's
 *
 * ERROR:
 * sqlite3.js: Ignoring inability to install OPFS sqlite3_vfs: The OPFS sqlite3_vfs cannot run in the main thread because it requires Atomics.wait().
 */

const log = console.log;

const worker = new Worker(chrome.runtime.getURL("worker.js"));

chrome.runtime.onConnect.addListener((port) => {
  if (port.name !== "devtools-db") return;
  log("[OFFSCREEN]: connected to devtools-db");

  port.onMessage.addListener(async (msg) => {
    log("[OFFSCREEN]: message from dev-tools-db: ", msg);
    worker.postMessage(msg);
  });

  worker.addEventListener("message", function (evt) {
    const { requestId, result } = evt.data; // { requestId, result, error }
    port.postMessage({ requestId, result, reply: true });
  });
});

chrome.runtime.onConnectExternal.addListener((port) => {
  if (port.name !== "page-script-fetcher") return;
  log("[OFFSCREEN]: connected to page-script-fetcher");

  port.onMessage.addListener(async (msg) => {
    log("[OFFSCREEN]: message from page-script-fetcher: ", msg);
  });
});

// chrome.runtime.onMessageExternal.addListener(
//   (request, sender, sendResponse) => {
//     worker.postMessage({
//       requestData: request.requestData,
//       responseData: request.responseData,
//     });
//   },
// );

// chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
//   console.log(
//     `request: ${request} sender: ${sender} sendResp: ${sendResponse}`,
//   );
//   worker.postMessage("global", {
//     request: request,
//     sender: sender,
//     sendResponse: sendResponse,
//   });
// });
