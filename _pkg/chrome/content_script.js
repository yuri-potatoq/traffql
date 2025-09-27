console.log("injecting request_wrapper_script");

const extensionId = chrome.runtime.id;
var s = document.createElement("script");
s.id = "request_wrapper_script";
s.src = chrome.runtime.getURL(
  `request_wrapper_script.js?extensionId=${extensionId}`,
);
(document.head || document.documentElement).appendChild(s);
