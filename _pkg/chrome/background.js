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

(async () => {
  await chrome.offscreen.createDocument({
    url: "offscreen.html",
    reasons: [chrome.offscreen.Reason.WORKERS],
    justification: "use OPFS to host sqlite3",
  });
  console.log("offscrean created");
})();
