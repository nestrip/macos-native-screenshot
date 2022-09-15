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

  window.__TAURI__.event.listen("WINDOW_CLOSE_REQUESTED", () => {
    if (apiKey.value == "") {
      window.__TAURI__.window.close();
      console.log("Hi");
    }
  });
});
