#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    fs::{self, rename},
    io,
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
    config
        .convert_options
        .iter()
        .try_for_each(|it| match it.as_str() {
            "world" => convert_world(&config.root_dir, &config.uuids),
            "plugin" => convert_plugin(&config.root_dir, &config.uuids),
            _ => return Err(format!("Unknown convert option: {}", it)),
        })?;
    Ok(())
}

fn convert_world(root_dir: &str, entries: &HashMap<String, String>) -> Result<(), String> {
    let worlds = scan_worlds(root_dir).map_err(|it| it.to_string())?;
    worlds
        .iter()
        .try_for_each(|it| rename_all_files_in_dir(it, entries).map_err(|it| it.to_string()))?;
    Ok(())
}

fn convert_plugin(root_dir: &str, entries: &HashMap<String, String>) -> Result<(), String> {
    println!("{:?}", scan_plugins(root_dir));
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

fn rename_all_files_in_dir<P: AsRef<Path>>(
    dir: P,
    entries: &HashMap<String, String>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            rename_all_files_in_dir(&path, entries)?;
            continue;
        }
        if let Some(name) = path.file_name() {
            if let Some(from) = entries
                .keys()
                .find(|k| name.to_str().unwrap().starts_with(*k))
            {
                let new_path = path.with_file_name(
                    entries[from].clone() + &name.to_str().unwrap()[from.len()..].to_string(),
                );
                fs::rename(path, new_path)?;
            } else {
                continue;
            }
        }
    }
    Ok(())
}
