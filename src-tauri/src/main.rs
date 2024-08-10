// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod icon;
mod img;

use anyhow::{Context as _, Result};
use itertools::Itertools;
use komorebi_client::{send_query, SocketMessage, State};
use komorebi_client::{Monitor, Window, Workspace};
use tauri::Manager as _;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri_plugin_positioner::{Position, WindowExt};
use winput::{
    message_loop::{self, EventReceiver},
    Action, Vk,
};

#[tokio::main]
async fn main() -> Result<()> {
    let tray_menu = SystemTrayMenu::new();

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(tray)
        .on_system_tray_event(move |app, event| match event {
            // TODO: 関数へ切り出し。とりあえずeventだけ投げれば良さそう
            // SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            //     "quit" => {
            //         // *loop_break_system_tray.write().unwrap() = true;
            //         // app.exit(0);
            //     }
            //     _ => {}
            // },
            SystemTrayEvent::DoubleClick { .. } => {
                // ダブルクリックで終了
                app.exit(0);
            }
            _ => {}
        })
        .setup(|app| {
            let main_window_alt = app.get_window("main").unwrap();
            let app_handle = app.app_handle();

            main_window_alt.hide().unwrap();
            main_window_alt.move_window(Position::TopCenter).unwrap();

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

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
                            // Windowを表示して描画。イベントを送る。
                            app_handle
                                    .emit_all("re-rendering", fetch_asayake_window_state(0))
                                    .unwrap();
                            main_window_alt.show().unwrap();
                        } else {
                            // Windowを隠す
                            main_window_alt.hide().unwrap();
                        }
                    }
                }
            });

            let main_window_notify = app.get_window("main").unwrap();
            let app_handle_notify = app.app_handle();

            tokio::task::spawn(async move {
                // komorebiの通知の監視
                // TODO: 関数へ切り出し
                // メモ: asyncブロックはfutureを返す。だからmoveしている変数だけ引数にしてasync関数をつくればいい
                let notify_receiver = komorebi_client::subscribe("asayake")
                    .context("Unable to subscribe notifyes from komorebi now.")
                    .unwrap();

                for incoming in notify_receiver.incoming() {
                    match incoming {
                        Ok(_) => {
                            if main_window_notify.is_visible().unwrap() {
                                // フロントエンド側に再レンダリングを要求
                                app_handle_notify
                                    .emit_all("re-rendering", fetch_asayake_window_state(0))
                                    .unwrap();
                            }
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![fetch_asayake_window_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

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
// TODO: timeoutの実装
fn fetch_komorebi_state() -> Result<State> {
    sonic_rs::from_str(&send_query(&SocketMessage::State).context("Unable to get the state of komorebi now.")?)
        .context("Unable to parse the state of komorebi.")
    // If you get this error.
    // You may be running a different versionof komorebi (We're using komorebi-client v0.1.28)
    // or, you may not running komorebi
}

// HACK: クレートを分割するとimplできないという制約を乗り越えるための方法。醜い
include!("./structs.rs");

impl From<&Monitor> for AsayakeMonitorState {
    fn from(value: &Monitor) -> Self {
        let workspaces = value.workspaces();
        let focusing_workspace = value.focused_workspace_idx();

        let mut workspaces_for_send: Vec<WorkspaceForSend> = vec![];

        let mut index = 0;
        for woi in workspaces {
            let mut workspace: WorkspaceForSend = woi.into();
            if index == focusing_workspace {
                workspace.focusing = true;
            }
            workspaces_for_send.push(workspace);
            index += 1;
        }

        AsayakeMonitorState {
            monitor_id: value.id(),
            focusing_workspace: focusing_workspace,
            workspaces: workspaces_for_send,
            size: value.size().into()
        }
    }
}

impl From<&Workspace> for WorkspaceForSend {
    fn from(value: &Workspace) -> Self {
        let mut container_for_send: Vec<ContainerForSend> = vec![];

        let containers = value.containers();

        for coni in containers {
            container_for_send.push(coni.into());
        }

        WorkspaceForSend {
            items: container_for_send,
            layout: value.latest_layout().iter().map(|x| x.into()).collect_vec(),
            focusing: false
        }
    }
}

impl From<&komorebi_client::Rect> for Rect {
    fn from(value: &komorebi_client::Rect) -> Self {
        Rect {
            left_top: (value.left as u16, value.top as u16),
            right_bottom: (value.right as u16, value.bottom as u16),
        }
    }
}

impl From<&komorebi_client::Container> for ContainerForSend {
    fn from(value: &komorebi_client::Container) -> Self {
        let mut window_for_send: Vec<WindowForSend> = vec![];

        let windows = value.windows();

        for wini in windows {
            window_for_send.push(wini.into());
        }

        ContainerForSend {
            windows: window_for_send,
        }
    }
}

impl From<&Window> for WindowForSend {
    fn from(value: &Window) -> Self {
        value.hwnd().into()
    }
}

/// KomorebiのStateをWindow向けに処理して返します
/// * `window_num` zero-based indeex
// TODO: Fromトレイトを実装して`From<&Monitor> for AsayakeMonitorState`を使って変換するようにする
#[tauri::command]
fn fetch_asayake_window_state(window_num: u64) -> AsayakeMonitorState {
    // komorebiの状態から自分のモニターを抜き出す
    let komorebi_state = fetch_komorebi_state().unwrap();
    let monitor = komorebi_state.monitors.elements().get(window_num as usize).unwrap();

    monitor.into()
}

#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};

    use crate::{fetch_asayake_window_state, fetch_komorebi_state};
    use std::fs::File;
    use std::io::Write;

    #[test]
    #[ignore]
    fn test_fetch_asayake_window_state_run() -> Result<()> {
        let mut output = File::create("example_asayake_state.dpp")?;
        write!(output, "{:?}", fetch_asayake_window_state(0))?;
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_fetch_komorebi_state() -> Result<()> {
        fetch_komorebi_state().unwrap();
        Ok(())
    }
}
