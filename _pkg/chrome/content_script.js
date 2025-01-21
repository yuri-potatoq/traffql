console.log("injecting request_wrapper_script");

const extensionId = chrome.runtime.id;

window.addEventListener(
  "interceptedFetch",
  async function (evt) {
    console.log("interceptedFetch received!");

    chrome.runtime.sendMessage(extensionId, {}, function (response) {
      console.log(response);
    });
  },
  false,
);

var s = document.createElement("script");
s.id = "request_wrapper_script";
s.src = chrome.runtime.getURL(`request_wrapper_script.js?extensionId=${extensionId}`);
(document.head || document.documentElement).appendChild(s);

