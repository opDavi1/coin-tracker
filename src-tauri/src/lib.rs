// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>

pub mod coin;
pub mod database;
pub mod state;

use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Builder, Manager, State};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn db_delete_coin(app_handle: AppHandle, id: i64) -> Result<usize, String> {
    app_handle.db(|db| database::delete_coin(db, &id)).map_err(|err| err.to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .manage(AppState {db: Default::default() })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            db_delete_coin,
            ])
        .setup(|app| {
            let handle = app.handle();
            let app_state : State<AppState> = handle.state();
            let db = database::init(&handle, None).expect("Could not initialize database");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    
}
