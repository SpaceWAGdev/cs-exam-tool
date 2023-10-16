import { invoke } from "@tauri-apps/api/tauri";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}
window.addEventListener("DOMContentLoaded", async () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  let is_elevated : Boolean = await invoke("get_admin_permissions");
  document.querySelector("#admin_rights")!.innerHTML = is_elevated == true? "Admin" : "Normal User";

  let process_list : string[] = await invoke("get_process_list");
  document.querySelector("#process_list")!.innerHTML = process_list.join("\n");
});

document.addEventListener('contextmenu', event => event.preventDefault());