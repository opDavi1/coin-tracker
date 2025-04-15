// This file was taken from RandomEngy/tauri-sqlite on github, which has no license listed.
// If anybody has an issue with this, please contact me

use rusqlite::Connection;
use tauri::{AppHandle, State, Manager};

pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
}

pub trait ServiceAccess {
  fn db<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult;

  fn db_mut<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult;
}

impl ServiceAccess for AppHandle {
  fn db<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult {
    let app_state: State<AppState> = self.state();
    let db_connection_guard = app_state.db.lock().unwrap();
    let db = db_connection_guard.as_ref().unwrap();
  
    operation(db)
  }

  fn db_mut<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult {
    let app_state: State<AppState> = self.state();
    let mut db_connection_guard = app_state.db.lock().unwrap();
    let db = db_connection_guard.as_mut().unwrap();
  
    operation(db)
  }
}