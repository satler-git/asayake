use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct AsayakeMonitorState {
    pub monitor_id: isize,
    pub focusing_workspace: usize,
    pub workspaces: Vec<WorkspaceForSend>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct WorkspaceForSend {
    pub items: Vec<ContainerForSend>,
    pub layout: LayoutForSend
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct ContainerForSend {
    pub windows: Vec<WindowForSend>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct WindowForSend {
    pub id: u64,
    pub icon: Icon,
    pub accent_color: u32
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Icon {
    pub base64_icon: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum LayoutForSend {
    Default(DefaultLayout),
    // Custom(CustomLayout),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum DefaultLayout {
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
