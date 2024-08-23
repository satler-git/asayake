use gloo::console::log;
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
                        } else if let Err(err) = res.payload {
                            // エラーの場合は飛んできたエラーをセットする。
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
            .map(|x| html! { <Workspace workspace={x.clone()} size={asayake_state.size.clone()} />})
            .collect::<Vec<Html>>()
    });

    let err = use_memo(asayake_error_state, |asayake_error_state| {
        match asayake_error_state.as_ref() {
            Some(err) => html! {
                <div class="error">{format!("{err}")}</div>
            },
            None => html!{},
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

#[derive(yew::Properties, PartialEq, Debug, Serialize)]
pub struct WorkspaceProps {
    pub workspace: WorkspaceForSend,
    pub size: Rect,
}

// fn diff(x: &u16, y: &u16) -> u16 {
//     // std::cmp::max(x, y) - std::cmp::min(x, y)
//     x - y
// }

/// (style, 縦の最小, 横の最小)
fn calc_grid_parent(layout: &Vec<Rect>, size: &Rect) -> (String, u16, u16) {
    // 基本的には一番小さいものを求めてそれでモニターのサイズを割る。それによってグリッドを作る
    let min_v = {
        // y座標(row向け)
        let min_rect = layout
            .iter()
            .min_by_key(|&re| re.right_bottom.1 - re.left_top.1)
            .unwrap();
        min_rect.right_bottom.1 - min_rect.left_top.1
    };
    let min_h = {
        // x座標(colum向け)
        let min_rect = layout
            .iter()
            .min_by_key(|&re| re.right_bottom.1 - &re.left_top.1)
            .unwrap();
        min_rect.right_bottom.0 - min_rect.left_top.0
    };
    let rows = (size.right_bottom.1 - size.left_top.1) / min_v;
    let colums = (size.right_bottom.0 - size.left_top.0) / min_h;

    let workspace_style = format!(
        "grid-template-rows: {};grid-template-columns: {};",
        vec!["1fr"; rows as usize].join(" "),
        vec!["1fr"; colums as usize].join(" ")
    );
    (workspace_style, min_h, min_v)
}

/// * `min` (row, colum)
fn calc_grid_child(min: &(u16, u16), win: &Rect) -> String { // (win.lt / min) + 1, (win.lt / min + 1) + ((win.rb - win.lt) / min) - 1
    let column_start = win.left_top.0 / min.0 + 1;
    let row_start = win.left_top.1 / min.1 + 1;
    format!(
        "grid-row: {} / {};grid-column: {} / {}",
        row_start,
        row_start + (( win.right_bottom.1 - win.left_top.1) / min.1) -1,
        column_start, // rowの開始のセル。1から始まるけど座標は(0, 0)からだから+1
        // rowの終了セル。最初のにwindowの大きさがminより大きければ足すけど、minと同じだと1 + 1みたいな感じで大きくなるから-1
        column_start + (( win.right_bottom.0 - win.left_top.0) / min.0) -1,
    )
}

#[function_component]
fn Workspace(props: &WorkspaceProps) -> Html {
    // log!(serde_json::to_string(props).unwrap()); // TO/DO: 削除
    let grid = calc_grid_parent(&props.workspace.layout, &props.size);

    let containers = props
        .workspace
        .items
        .iter()
        .zip(props.workspace.layout.iter())
        .map(|x| {
            let style = calc_grid_child(&(grid.1, grid.2), x.1);
            log!("calc child");
            html! {
                <div style={style}>
                    <Container windows={x.0.windows.clone()}/>
                </div>
            }
        })
        .collect::<Vec<Html>>();

    html! {
        <div class="workspace" style={grid.0}>
            if props.workspace.focusing {
                <div class="selector"/>
            }
            {containers}
        </div>
    }
}

#[function_component]
fn Container(container: &ContainerForSend) -> Html {
    // TODO: 5以上だと表示が変になる
    let icons = container
        .windows
        .iter()
        .map(|x| html! { <img src={x.icon.base64_icon.clone()} class="icon" />})
        .collect::<Vec<Html>>();

    html! {
        <div class="container" style={
            format!("border-color:#{};", container.windows[0].accent_color)
        }>
            {icons}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use crate::app::{calc_grid_child, Rect};

    use super::calc_grid_parent;

    fn make_rect(right_bottom: (u16, u16)) -> Rect {
        Rect { left_top: (0, 0), right_bottom: right_bottom }
    }

    #[test]
    fn test_calc_grid() {
        assert_eq!(
            calc_grid_parent(&vec![
                Rect { left_top: (0, 0), right_bottom: (1, 1) },
                Rect { left_top: (1, 0), right_bottom: (2, 1) },
            ], &make_rect((2, 1))),
            ("grid-template-rows: 1fr;grid-template-columns: 1fr 1fr;".into(), 1, 1)
        );
    }

    #[test]
    fn test_calc_child() {
        assert_eq!(
            calc_grid_child(&(1, 1), &Rect { left_top: (0, 0), right_bottom: (1, 1) }),
            "grid-row: 1 / 1;grid-column: 1 / 1"
        );
        assert_eq!(
            calc_grid_child(&(1, 1), &Rect { left_top: (1, 0), right_bottom: (2, 1) }),
            "grid-row: 1 / 1;grid-column: 2 / 2"
        );
    }
}
