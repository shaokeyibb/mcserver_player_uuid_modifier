use std::{fs::File, io::BufReader};

use native_dialog::FileDialog;
use serde::{Deserialize, Serialize};

#[tauri::command]
pub fn open_dir_dialog() -> Result<String, String> {
    FileDialog::new()
        .show_open_single_dir()
        .map(|path| path.map_or_else(|| String::from(""), |it| it.to_str().unwrap().into()))
        .map_err(|err| err.to_string())
}

#[derive(Serialize, Deserialize)]
pub struct UserCache {
    name: String,
    uuid: String,
    #[serde(rename = "expiresOn")]
    expires_on: String,
}

#[tauri::command]
pub fn read_usercache(rootdir: &str) -> Result<Vec<UserCache>, String> {
    let file = File::open(rootdir.to_owned() + "/usercache.json").map_err(|err| err.to_string())?;
    let reader = BufReader::new(file);
    serde_json::from_reader::<_, Vec<UserCache>>(reader).map_err(|err| err.to_string())
}
