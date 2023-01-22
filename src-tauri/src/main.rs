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
fn convert(config: Config) -> Result<Vec<PathBuf>, String> {
    let mut result = Vec::new();
    for it in config.convert_options.iter() {
        match it.as_str() {
            "world" => {
                convert_worlds(&config.root_dir, &config.uuids)
                    .into_iter()
                    .for_each(|it| {
                        result.extend(it);
                    });
            }
            "plugin_text" => {
                convert_plugins(&config.root_dir, &config.uuids)
                    .into_iter()
                    .for_each(|it| {
                        result.extend(it);
                    });
            }
            _ => return Err(format!("Unknown convert option: {}", it)),
        }
    }
    Ok(result)
}

fn convert_worlds(
    root_dir: &str,
    entries: &HashMap<String, String>,
) -> Result<Vec<PathBuf>, String> {
    let worlds = scan_worlds(root_dir).map_err(|it| it.to_string())?;
    let result = worlds
        .iter()
        .map(|it| rename_all_files_in_dir(it, entries))
        .filter(|it| it.is_ok())
        .flat_map(|it| it.unwrap())
        .collect();
    Ok(result)
}

fn convert_plugins(
    root_dir: &str,
    entries: &HashMap<String, String>,
) -> Result<Vec<PathBuf>, String> {
    println!("{:?}", scan_plugins(root_dir));
    Ok(Vec::new())
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
) -> io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let rst = rename_all_files_in_dir(&path, entries)?;
            result.extend(rst);
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
                fs::rename(path, &new_path)?;
                result.push(new_path);
            } else {
                continue;
            }
        }
    }
    Ok(result)
}

fn rename_all_files_text_in_dir<P: AsRef<Path>>(
    dir: P,
    entries: &HashMap<String, String>,
) -> io::Result<Vec<PathBuf>> {
    Ok(Vec::new())
}
