use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;

use wasm_bindgen::prelude::*;

use yew::{platform::spawn_local, prelude::*, virtual_dom::VNode};

include!("./structs.rs");

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Event {
    pub payload: Result<AsayakeMonitorState, AsayakeError>,
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
    // エラー処理はバッドプラクティス
    // ただしUseStateHandleが面倒くさいせいでこうなった
    let asayake_state = use_state_eq(|| AsayakeMonitorState::default());
    let asayake_error_state = use_state_eq(|| None);

    {
        let asayake_state = asayake_state.clone();
        let asayake_error_state = asayake_error_state.clone();
        use_effect_with((), move |_| { // ()を使用することで最初だけ実行するようになる
            let asayake_state = asayake_state.clone();
            let asayake_error_state = asayake_error_state.clone();
            spawn_local(async move {
                let handler = Closure::<dyn FnMut(_)>::new(move |s: JsValue| {
                    if let Ok(res) = from_value::<Event>(s) {
                        if let Ok(payload) = res.payload {
                            asayake_state.set(payload);
                        } else if let Err(err) = res.payload { // エラーの場合は飛んできたエラーをセットする。
                            asayake_state.set(AsayakeMonitorState::default());
                            asayake_error_state.set(Some(err));
                        }
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
            .map(|x| html! { <Workspace items={x.items.clone()} layout={x.layout.clone()} focusing={x.focusing.clone()} />})
            .collect::<Vec<Html>>()
    });

    let err = use_memo(asayake_error_state, |asayake_error_state| {
        let str = match asayake_error_state.as_ref() {
            Some(err) => format!("{err}"),
            None => "".into(),
        };
        html! {
            <div class="error">{str}</div>
        }
    });

    html! {
        <div class="baord">
        <div class="flex">
                {<Vec<VNode> as Clone>::clone(&*workspaces)}
                {<VNode as Clone>::clone(&*err)}
            </div>
        </div>
    }
}

#[function_component]
fn Workspace(workspace: &WorkspaceForSend) -> Html {
    html! {
        <div class="workspace">
            if workspace.focusing {
                <div class="selector"/>
            }
        </div>
    }
}

#[function_component]
fn Container(container: &ContainerForSend) -> Html {
    html! {
        <div class="container">
        </div>
    }
}
