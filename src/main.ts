import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";


window.addEventListener("DOMContentLoaded", async () => {
  let is_elevated : Boolean = await invoke("get_admin_permissions");
  document.querySelector("#admin_rights")!.innerHTML = is_elevated == true? "Admin" : "Normal User";

  let process_list : string[] = await invoke("get_process_list");
  document.querySelector("#process_list")!.innerHTML = process_list.join("\n");
});

document.addEventListener('contextmenu', event => event.preventDefault());