use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct AsayakeMonitorState {
    monitor_id: isize,
    focusing_workspace: usize,
    workspaces: Vec<WorkspaceForSend>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct WorkspaceForSend {
    items: Vec<ContainerForSend>,
    layout: LayoutForSend
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct ContainerForSend {
    windows: Vec<WindowForSend>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct WindowForSend {
    icon: Icon,
    accent_color: u32
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Icon {
    base64_icon: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
enum LayoutForSend {
    Default(DefaultLayout),
    // Custom(CustomLayout),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
enum DefaultLayout {
    BSP,
    Columns,
    Rows,
    VerticalStack,
    HorizontalStack,
    UltrawideVerticalStack,
    Grid,
    RightMainVerticalStack,
}

// Maybe-later: CustomLayoutは実装が難しそう
// #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
// struct CustomLayout(Vec<Column>);
