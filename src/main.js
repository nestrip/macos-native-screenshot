const { invoke } = window.__TAURI__.tauri;

let button;

window.addEventListener("DOMContentLoaded", () => {
  button = document.getElementById("test");

  button.addEventListener("click", () => {
    notification();
  });
});

function notification() {
  invoke("test");
}

window.notification = notification;
