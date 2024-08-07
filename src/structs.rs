#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
pub struct AsayakeMonitorState {
    pub monitor_id: isize,
    pub focusing_workspace: usize,
    pub workspaces: Vec<WorkspaceForSend>,
}

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash, yew::Properties,
)]
pub struct WorkspaceForSend {
    pub items: Vec<ContainerForSend>,
    pub layout: LayoutForSend,
}

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash, yew::Properties
)]
pub struct ContainerForSend {
    pub windows: Vec<WindowForSend>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
pub struct WindowForSend {
    pub id: u16,
    pub icon: Icon,
    pub accent_color: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
pub struct Icon {
    pub base64_icon: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
pub enum LayoutForSend {
    Default(DefaultLayout),
    // Custom(CustomLayout),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
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

impl Default for AsayakeMonitorState {
    fn default() -> Self {
        AsayakeMonitorState {
            monitor_id: 0,
            focusing_workspace: 0,
            workspaces: vec![],
        }
    }
}

impl Default for ContainerForSend {
    fn default() -> Self {
        ContainerForSend {
            windows: vec![],
        }
    }
}

impl Default for WindowForSend {
    fn default() -> Self {
        WindowForSend {
            id: 0,
            icon: Icon::default(),
            accent_color: 0xFFFFFF,
        }
    }
}

impl Default for Icon {
    fn default() -> Self {
        Icon {
            base64_icon: "".into(),
        }
    }
}

// Maybe-later: CustomLayoutは実装が難しそう
// #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
// struct CustomLayout(Vec<Column>);
