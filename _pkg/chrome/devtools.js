chrome.devtools.panels.create(
  "TraffQL Panel",
  "MyPanelIcon.png",
  "devtools_panel.html",
  function (panel) {
    // code invoked on panel creation
    console.log("panel invoked!");
  },
);
