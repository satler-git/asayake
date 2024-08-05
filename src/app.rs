use anyhow::{Context as _, Result};

use gloo_utils::format::JsValueSerdeExt;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use wasm_bindgen::prelude::*;

use yew::{platform::{spawn_local, time::sleep}, prelude::*, virtual_dom::VNode};

use std::time::Duration;

include!("./structs.rs");

const ONE_SEC: Duration = Duration::from_secs(1);

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Event {
    pub payload: AsayakeMonitorState,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &js_sys::Function) -> JsValue;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct FetchAsayakeStateArgs {
    #[serde(rename = "windowNum")]
    window_num: u64,
}

async fn fetch_asayake_state(window_num: u64) -> Result<AsayakeMonitorState> {
    let args = to_value(&FetchAsayakeStateArgs { window_num }).unwrap();
    let asayake_monitor_state: Result<AsayakeMonitorState, _> =
        invoke("fetch_asayake_window_state", args)
            .await
            .into_serde();
    asayake_monitor_state.context("Unable to fetch the state of asayake")
}

#[function_component()]
pub fn App() -> Html {
    let asayake_state = use_state(|| AsayakeMonitorState::default());

    let first = use_state(|| true);

    if *first {
        first.set(false);
        let asayake_state = asayake_state.clone();
        spawn_local(async move {
            loop {
                asayake_state.set(fetch_asayake_state(0).await.unwrap());
                sleep(ONE_SEC).await;
            }
        })
    }

    let workspaces = use_memo(asayake_state, |asayake_state| {
        asayake_state
            .workspaces
            .iter()
            .filter(|x| x.items != vec![])
            .map(|x| html! { <Workspace items={x.items.clone()} layout={x.layout.clone()} />})
            .collect::<Vec<Html>>()
    });

    html! {
        <div class="container">
            {<Vec<VNode> as Clone>::clone(&*workspaces.clone())}
        </div>
    }
}

#[function_component]
fn Workspace(workspace: &WorkspaceForSend) -> Html {
    html! {
        <div class="workspace">
        </div>
    }
}

#[function_component]
fn Container(container: &ContainerForSend) -> Html {
    todo!()
}
