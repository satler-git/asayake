use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct AsayakeMonitorState {
    pub monitor_id: isize,
    pub focusing_workspace: usize,
    pub workspaces: Vec<WorkspaceForSend>,
    pub size: Rect,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct WorkspaceForSend {
    pub items: Vec<ContainerForSend>,
    pub layout: Vec<Rect>,
    pub focusing: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct ContainerForSend {
    pub windows: Vec<WindowForSend>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct WindowForSend {
    pub id: u16,
    pub icon: Icon,
    pub accent_color: u32
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Icon {
    pub base64_icon: String,
}

/// TODO: komorebi_clientの実装を見るとマイナスの可能性もあるかも
#[derive(PartialEq, Eq, Debug, Clone, serde::Deserialize, serde::Serialize, Hash)]
pub struct Rect {
    left_top: (u16, u16),
    right_bottom: (u16, u16),
}

// Maybe-later: CustomLayoutは実装が難しそう
// #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
// struct CustomLayout(Vec<Column>);
