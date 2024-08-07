use anyhow::{Context as _, Result};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;

use wasm_bindgen::prelude::*;

use yew::{
    platform::spawn_local,
    prelude::*,
    virtual_dom::VNode,
};

include!("./structs.rs");

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

#[function_component()]
pub fn App() -> Html {
    let asayake_state = use_state_eq(|| AsayakeMonitorState::default());

    {
        let asayake_state = asayake_state.clone();
        use_effect_with((), move |_| {
            let asayake_state = asayake_state.clone();
            spawn_local(async move {
                let handler = Closure::<dyn FnMut(_)>::new(move |s: JsValue| {
                    if let Ok(res) = from_value::<Event>(s) {
                        asayake_state.set(res.payload);
                    }
                });
                let _ = listen("re-rendering", handler.as_ref().unchecked_ref()).await;
                handler.forget();
            })
        });
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
