#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lyrics;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn search_lyrics(query: &str) -> Result<String, String> {
    let lyrics = match lyrics::fetch_lyrics(query).await {
        Ok(lyrics) => lyrics,
        Err(_) => String::from("Error while trying to fetch lyrics."),
    };

    Ok(lyrics)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_lyrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
