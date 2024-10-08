// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod icon;
mod img;

use anyhow::{Context as _, Result, anyhow};
use komorebi_client::{send_query, Layout, SocketMessage, State};
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
            let main_window_notify = app.get_window("main").unwrap();

            tokio::task::spawn(async move {
                // Altキーの監視
                // TODO: 関数へ切り出し
                // メモ: asyncブロックはfutureを返す。だからmoveしている変数だけ引数にしてasync関数をつくればいい
                let receiver = message_loop::start().unwrap();
                let mut state = alt_state(&receiver, &false).unwrap();
                main_window_alt.hide().unwrap();
                main_window_alt.move_window(Position::TopCenter).unwrap();

                loop {
                    let old_state = state.clone();
                    state = alt_state(&receiver, &old_state).unwrap();
                    if old_state != state {
                        if state {
                            // Windowを表示して描画。イベントを送る。
                            main_window_alt.show().unwrap();
                        } else {
                            // Windowを隠す
                            main_window_alt.hide().unwrap();
                        }
                    }
                }
            });

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
        .invoke_handler(tauri::generate_handler![greet, fetch_asayake_window_state])
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
fn fetch_komorebi_state() -> Result<State> {
    sonic_rs::from_str(&send_query(&SocketMessage::State)?)
        .context("Unable to get the state of komorebi now.")
    // If you get this error.
    // You may be running a different versionof komorebi (We're using komorebi-client v0.1.28)
    // or, you may not running komorebi
}

// HACK: クレートを分割するとimplできないという制約を乗り越えるための方法。醜い
include!("./structs.rs");

impl TryFrom<&Monitor> for AsayakeMonitorState {
    type Error = anyhow::Error;

    fn try_from(value: &Monitor) -> Result<Self, Self::Error> {
        let workspaces = value.workspaces();
        let focusing_workspace = value.focused_workspace_idx();

        let mut workspaces_for_send: Vec<WorkspaceForSend> = vec![];

        for woi in workspaces {
            workspaces_for_send.push(woi.try_into()?);
        }

        Ok(AsayakeMonitorState {
            monitor_id: value.id(),
            focusing_workspace: focusing_workspace,
            workspaces: workspaces_for_send,
        })
    }
}

impl TryFrom<&Workspace> for WorkspaceForSend {
    type Error = anyhow::Error;

    fn try_from(value: &Workspace) -> Result<Self, Self::Error> {
        let mut container_for_send: Vec<ContainerForSend> = vec![];

        let containers = value.containers();

        for coni in containers {
            container_for_send.push(coni.try_into()?);
        }

        Ok(WorkspaceForSend {
            items: container_for_send,
            layout: value.layout().try_into()?,
        })
    }
}

impl TryFrom<&Layout> for LayoutForSend {
    type Error = anyhow::Error;

    fn try_from(value: &Layout) -> Result<Self, Self::Error> {
        if let Layout::Default(default_layout_kind) = value {
            Ok(LayoutForSend::Default(match default_layout_kind {
                komorebi_client::DefaultLayout::BSP => DefaultLayout::BSP,
                komorebi_client::DefaultLayout::Columns => DefaultLayout::Columns,
                komorebi_client::DefaultLayout::Rows => DefaultLayout::Rows,
                komorebi_client::DefaultLayout::VerticalStack => DefaultLayout::VerticalStack,
                komorebi_client::DefaultLayout::HorizontalStack => DefaultLayout::HorizontalStack,
                komorebi_client::DefaultLayout::UltrawideVerticalStack => {
                    DefaultLayout::UltrawideVerticalStack
                }
                komorebi_client::DefaultLayout::Grid => DefaultLayout::Grid,
                komorebi_client::DefaultLayout::RightMainVerticalStack => {
                    DefaultLayout::RightMainVerticalStack
                }
            }))
        } else {
            Err(anyhow!("Unable to parse custom layout, asayake still doesn't have compatibility for custom layout"))
        }
    }
}

impl TryFrom<&komorebi_client::Container> for ContainerForSend {
    type Error = anyhow::Error;

    fn try_from(value: &komorebi_client::Container) -> Result<Self, Self::Error> {
        let mut window_for_send: Vec<WindowForSend> = vec![];

        let windows = value.windows();

        for wini in windows {
            window_for_send.push(wini.try_into()?);
        }

        Ok(ContainerForSend {
            windows: window_for_send,
        })
    }
}

impl TryFrom<&Window> for WindowForSend {
    type Error = anyhow::Error;

    fn try_from(value: &Window) -> Result<Self, Self::Error> {
        Ok(value.hwnd().try_into()?)
    }
}

/// KomorebiのStateをWindow向けに処理して返します
/// * `window_num` zero-based indeex
// TODO: Fromトレイトを実装して`From<&Monitor> for AsayakeMonitorState`を使って変換するようにする
#[tauri::command]
fn fetch_asayake_window_state(window_num: usize) -> AsayakeMonitorState {
    // komorebiの状態から自分のモニターを抜き出す
    let komorebi_state = fetch_komorebi_state().unwrap();
    let monitor = komorebi_state.monitors.elements().get(window_num).unwrap();

    monitor.try_into().unwrap()
}
