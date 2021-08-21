#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod blocklist;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![fetch_blocklist, add_to_blocklist])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn add_to_blocklist(site: String) -> Result<String, String> {
  println!("add_to_blocklist");
  println!("{}", site);
  blocklist::add_to_blocklist(site.as_str());
  Ok(site)
}

#[tauri::command]
fn fetch_blocklist() -> Result<Vec<String>, String> {
  println!("fetch_blocklist");
  let bar = blocklist::fetch_blocklist();
  // Err("this is an error!!".to_string())
  println!("{:?}", bar);
  Ok(bar)
}
