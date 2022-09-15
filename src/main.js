window.addEventListener("DOMContentLoaded", () => {
  const apiKey = document.getElementById("apiKey");
  window.__TAURI__.invoke("get_api_key").then((key) => {
    apiKey.value = key;
  });

  apiKey.onchange = (e) => {
    window.__TAURI__.invoke("set_api_key", { apiKey: e.target.value });
  };

  const testButton = document.getElementById("test");

  testButton.onclick = async () => {
    window.__TAURI__.invoke("test_upload");
  };

  window.__TAURI__.event.listen("tauri://close-requested", () => {
    // If they have an api key, just close the window
    if (apiKey.value != "") {
      window.__TAURI__.window.appWindow.close();
      return;
    }

    // display a warning about not having an api key
    window.__TAURI__.dialog.message("You don't have an api key.", {
      title: "No API Key",
      type: "error",
    });
  });
});
