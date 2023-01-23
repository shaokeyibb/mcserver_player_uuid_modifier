#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    fs::{self},
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
    let plugins = scan_plugins(root_dir).map_err(|it| it.to_string())?;
    let mut result = Vec::new();
    for it in plugins.iter() {
        let file = rename_all_files_in_dir(it, entries).map_err(|it| it.to_string())?;
        let dir = rename_all_dir(it, entries).map_err(|it| it.to_string())?;
        let text = rename_all_text(it, entries).map_err(|it| it.to_string())?;
        result.extend(file);
        result.extend(dir);
        result.extend(text);
    }
    Ok(result)
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
        if let Some(name) = path.file_stem() {
            if let Some(from) = entries.keys().find(|k| name.to_str().unwrap_or("") == *k) {
                let new_path = path.with_file_name(
                    entries[from].clone()
                        + &path.extension().map_or(String::from(""), |it| {
                            String::from(".") + it.to_str().unwrap()
                        }),
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

fn rename_all_dir<P: AsRef<Path>>(
    dir: P,
    entries: &HashMap<String, String>,
) -> io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if let Some(name) = path.file_name() {
            if let Some(from) = entries.keys().find(|k| name.to_str().unwrap_or("") == *k) {
                let new_path = path.with_file_name(entries[from].clone());
                fs::rename(path, &new_path)?;
                result.push(new_path.clone());
                let rst = rename_all_dir(new_path, entries)?;
                result.extend(rst);
            } else {
                let rst = rename_all_dir(&path, entries)?;
                result.extend(rst);
                continue;
            }
        }
    }
    Ok(result)
}

fn rename_all_text<P: AsRef<Path>>(
    dir: P,
    entries: &HashMap<String, String>,
) -> io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let rst = rename_all_text(&path, entries)?;
            result.extend(rst);
            continue;
        }
        println!("Scanning text file: {:?}", path.clone());
        let read = fs::read_to_string(path.clone());
        // Slient ignore read_to_string error to skip binary files rea
        if read.is_err() {
            continue;
        }
        let mut read = read.unwrap();
        if entries.keys().filter(|k| read.contains(*k)).count() == 0 {
            continue;
        }
        entries
            .clone()
            .iter()
            .for_each(|(k, v)| read = read.replace(k, &v));
        fs::write(path.clone(), read)?;
        result.push(path);
    }
    Ok(result)
}
