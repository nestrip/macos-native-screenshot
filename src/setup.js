window.addEventListener("DOMContentLoaded", () => {
  let button = document.getElementById("setup");
  let apiKey = document.getElementById("apiKey");

  button.onclick = async () => {
    let key = apiKey.value;

    if (!key) {
      window.__TAURI__.dialog.message("The API Key is required.", {
        title: "API Key Required",
        type: "error",
      });
      return;
    }

    let correct = await window.__TAURI__.dialog.confirm(
      "Are you sure the api is correct?",
      {
        title: "Confirm API Key",
      }
    );

    if (!correct) {
      return;
    }

    await window.__TAURI__.invoke("set_api_key", { apiKey: key });

    let success = await window.__TAURI__.invoke("test_upload");

    if (success) {
      window.__TAURI__.invoke("set_setup");
      window.__TAURI__.process.relaunch();
    } else {
      window.__TAURI__.dialog.message("The api key is incorrect.", {
        title: "Incorrect API Key",
        type: "error",
      });
    }
  };
});
