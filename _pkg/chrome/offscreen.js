/**
 * Notes:
 *
 * Can't spawn sqlite instance using OPFS here because offscreen don't provide appropriated API's
 *
 * ERROR:
 * sqlite3.js: Ignoring inability to install OPFS sqlite3_vfs: The OPFS sqlite3_vfs cannot run in the main thread because it requires Atomics.wait().
 */

const log = console.log;

const worker = new Worker(chrome.runtime.getURL("worker.js"), {
  type: "module",
});

chrome.runtime.onConnect.addListener((port) => {
  if (port.name !== "devtools-worker") return;
  log("[OFFSCREEN]: connected to devtools-worker");

  port.onMessage.addListener(async (msg) => {
    log("[OFFSCREEN]: message from devtools-worker: ", msg);
    let { kind, payload } = msg;

    if (kind === "dump") {
      //TODO: move that to another place
      //only using here because devtools panel don't support open file picker
      const saveHandle = await window.showSaveFilePicker({
        suggestedName: "dump.sqlite3",
        types: [
          {
            description: "SQLite database",
            accept: { "application/vnd.sqlite3": [".sqlite3"] },
          },
        ],
      });
      payload = { ...payload, handle: saveHandle };
      msg = { ...msg, payload: payload };
    }

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
    worker.postMessage(msg);
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
