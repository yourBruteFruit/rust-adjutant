const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function loadRegistry() {
  await invoke("load_registry").then(
    (message) => {
      greetMsgEl.textContent = message
    }
  )
}

window.addEventListener("DOMContentLoaded", () => {
  loadRegistry()
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
});
