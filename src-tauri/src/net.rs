#[tauri::command]
pub fn fetch(url: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).send().map_err(|err| err.to_string())?;
    res.text().map_err(|err| err.to_string())
}

#[tauri::command]
pub fn fetch_post(url: &str, body: String) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(url)
        .body(body)
        .send()
        .map_err(|err| err.to_string())?;
    res.text().map_err(|err| err.to_string())
}
