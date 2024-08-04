use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::{AsayakeMonitorState, ContainerForSend, WorkspaceForSend};
use gloo_utils::format::JsValueSerdeExt;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Event {
    payload: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri", "event"])]
    async fn listen(event: &str, handler: &js_sys::Function) -> JsValue;
}

async fn fetch_asayake_state() -> Result<super::AsayakeMonitorState> {
    let asayake_monitor_state: Result<AsayakeMonitorState, _> =
        invoke("fetch_asayake_window_state", to_value(&()).unwrap())
            .await
            .into_serde();
    asayake_monitor_state.context("Unable to fetch the state of asayake")
}
