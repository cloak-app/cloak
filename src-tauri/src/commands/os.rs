#[tauri::command]
pub async fn reveal_item_in_dir(path: String) -> Result<(), String> {
    tauri_plugin_opener::reveal_item_in_dir(path).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_all_font_families() -> Result<Vec<String>, String> {
    let source = font_kit::source::SystemSource::new();
    let font_list = source.all_families().map_err(|e| e.to_string())?;
    Ok(font_list)
}
