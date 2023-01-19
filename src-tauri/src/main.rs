#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod calc;
mod file;
mod net;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            file::open_dir_dialog,
            file::read_usercache,
            calc::name_uuid_from_bytes,
            net::fetch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
