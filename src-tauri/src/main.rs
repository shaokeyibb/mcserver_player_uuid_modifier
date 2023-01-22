#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

mod calc;
mod file;
mod net;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            convert,
            file::open_dir_dialog,
            file::read_usercache,
            calc::name_uuid_from_bytes,
            net::fetch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(rename = "rootDir")]
    root_dir: String,
    #[serde(rename = "convertOptions")]
    convert_options: Vec<String>,
    uuids: HashMap<String, String>,
}

#[tauri::command]
fn convert(config: Config) -> Result<(), String> {
    println!("{:?}", scan_worlds(&config.root_dir));
    println!("{:?}", scan_plugins(&config.root_dir));
    Ok(())
}

fn scan_worlds<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let collect = fs::read_dir(path)?
        .filter_map(|it| {
            if let Ok(path) = it {
                path.path().join("level.dat").exists().then(|| path.path())
            } else {
                None
            }
        })
        .collect();
    Ok(collect)
}

fn scan_plugins<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let collect = fs::read_dir(path.as_ref().join("plugins"))?
    .filter_map(|it| {
        if let Ok(path) = it {
            path.path().is_dir().then(|| path.path())
        } else {
            None
        }
    })
    .collect();
    Ok(collect)
}
