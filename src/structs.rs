use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
struct AsayakeMonitorState {
    monitor_id: isize,
    focusing_workspace: usize,
    workspaces: Vec<WorkspaceForSend>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
struct WorkspaceForSend {
    items: Vec<ContainerForSend>,
    layout: LayoutForSend
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
struct ContainerForSend {
    windows: Vec<WindowForSend>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
struct WindowForSend {
    icon: Icon,
    accent_color: u32
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
struct Icon {
    base64_icon: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
enum LayoutForSend {
    Default(DefaultLayout),
    // Custom(CustomLayout),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
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
