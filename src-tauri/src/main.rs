// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, RwLock};

use anyhow::{Context as _, Result};

use komorebi_client::{send_query, SocketMessage, State};
use tauri::Manager as _;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use winput::{
    message_loop::{self, EventReceiver},
    Action, Vk,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() -> Result<()> {
    let loop_break = Arc::new(RwLock::new(false));

    let loop_break_rec = Arc::clone(&loop_break);
    let loop_break_system_tray = Arc::clone(&loop_break);

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(move |_app, event| match event { // TODO: 関数へ切り出し。とりあえずeventだけ投げれば良さそう
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    *loop_break_system_tray.write().unwrap() = true;
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            tokio::task::spawn(async move {
                // Altキーの監視
                // TODO: 関数へ切り出し
                // メモ: asyncブロックはfutureを返す。だからmoveしている変数だけ引数にしてasync関数をつくればいい
                let receiver = message_loop::start().unwrap();
                let mut state = alt_state(&receiver, &false).unwrap();

                loop {
                    let old_state = state.clone();
                    state = alt_state(&receiver, &old_state).unwrap();
                    if old_state != state {
                        if state {
                            // Windowを表示して描画。イベントを送る。windowの位置を正しくする。
                            main_window.show().unwrap();
                            main_window
                                .set_position(tauri::PhysicalPosition { x: 100, y: 100 })
                                .unwrap();
                        } else {
                            // Windowを隠す
                            main_window.hide().unwrap();
                        }
                    }
                    if *loop_break_rec.read().unwrap() {
                        break;
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    *loop_break.write().unwrap() = true;
    Ok(())
}

/// ALTキーの現在の状態を取得します
fn alt_state(receiver: &EventReceiver, old_state: &bool) -> Result<bool> {
    match receiver.next_event() {
        message_loop::Event::Keyboard {
            vk: Vk::Alt,
            action,
            ..
        } => match action {
            Action::Press => Ok(true),
            Action::Release => Ok(false),
        },
        _ => Ok(*old_state),
    }
}

/// komorebiのステータスを取得します
async fn komorebi_state() -> Result<State> {
    sonic_rs::from_str(&send_query(&SocketMessage::State)?)
        .context("Unable to get the state of komorebi now.")
    // If you get this error.
    // You may be running a different versionof komorebi (We're using komorebi-client v0.1.28)
    // or, you may not running komorebi
}
