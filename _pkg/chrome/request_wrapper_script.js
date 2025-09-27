let extensionId = new URL(
  document.getElementById("request_wrapper_script").src,
).searchParams.get("extensionId");
const port = chrome.runtime.connect(extensionId, {
  name: "page-script-fetcher",
});

window.fetch = new Proxy(window.fetch, {
  apply: function (target, that, args) {
    // https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch#syntax
    let temp = target.apply(that, args);
    let [resource, options] = args;
    options = options ?? {};

    temp.then(async (res) => {
      let internalResponse = res.clone();

      let responseData = {
        code: internalResponse.status,
        headers: internalResponse.headers,
        body: await internalResponse.text(),
      };

      let reqData = {
        url: resource.toString(),
        headers: options.headers,
        method: options.method,
        body: null,
      };

      if (options !== null) {
        if (typeof options.body === "string") {
          reqData.body = options.body;
        } else if (options.body instanceof Blob) {
          reqData.body = options.body.text();
        } else if (
          options.body instanceof ArrayBuffer ||
          options.body instanceof DataView
        ) {
          let decoder = new TextDecoder();
          reqData.body = decoder.decode(options.body);
        } else if (options.body instanceof FormData) {
          // TODO: build types of requests, for now only raw request and response body
        } else {
          console.error(
            `not suported type of ${typeof options.body}! url=${reqData.url} `,
          );
        }
      }

      port.postMessage({
        requestData: reqData,
        responseData: responseData,
      });

      // chrome.runtime.sendMessage(extensionId, );
      console.log(
        `FETCH: collected data request=${JSON.stringify(reqData)} response=${JSON.stringify(responseData)}`,
      );
    });
    return temp;
  },
});

XMLHttpRequest = new Proxy(XMLHttpRequest, {
  construct: function (target, args) {
    const xhr = new target(...args);
    // Do whatever you want with XHR request
    xhr.onreadystatechange = function () {
      if (xhr.readyState === XMLHttpRequest.DONE) {
        //  XMLHttpRequest.response can be an ArrayBuffer, a Blob, a Document, a JavaScript Object.
        let reqData = {
          method: xhr.status,
          url: xhr.responseURL,
        };

        let respData = {
          status: xhr.status,
          body: xhr.response,
          headers: xhr.getAllResponseHeaders(),
        };

        if (typeof respData.body === "string") {
          respData.body = respData.body;
        } else if (respData.body instanceof ArrayBuffer) {
          let decoder = new TextDecoder();
          respData.body = decoder.decode(respData.body);
        } else if (respData.body instanceof Blob) {
          respData.body = respData.body.text();
        } else {
          console.error(
            `XHR: not suported type of ${typeof respData.body}! url=${reqData.url} `,
          );
        }

        console.log(
          `XHR: collected data request=${JSON.stringify(reqData)} response=${JSON.stringify(respData)}`,
        );
      }
    };
    return xhr;
  },
});
