/**
 * Notes:
 * Can't spawn sqlite instance here because background in MV3 is an service worker, it is ephemeral
 *
 */

(async () => {
  if (await chrome.offscreen.hasDocument()) {
    console.warn("[BACKGROUND]: offscreen already exist");
  } else {
    await await chrome.offscreen.createDocument({
      url: "offscreen.html",
      reasons: [chrome.offscreen.Reason.WORKERS],
      justification: "use OPFS to host sqlite3",
    });
    console.log("offscrean created");
  }
})();

// chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
//   console.log(
//     sender.tab
//       ? "from a content script:" + sender.tab.url
//       : "from the extension",
//   );
//   if (request.greeting === "hello") sendResponse({ farewell: "goodbye" });
// });

// chrome.runtime.onMessageExternal.addListener(
//   function (request, sender, sendResponse) {
//     console.log("from external page");
//   },
// );
