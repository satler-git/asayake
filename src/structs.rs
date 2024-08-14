use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
pub struct AsayakeMonitorState {
    pub monitor_id: isize,
    pub focusing_workspace: usize,
    pub workspaces: Vec<WorkspaceForSend>,
    pub size: Rect,
}

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash, yew::Properties,
)]
pub struct WorkspaceForSend {
    pub items: Vec<ContainerForSend>,
    pub layout: Vec<Rect>,
    pub focusing: bool,
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

#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct Rect {
    left_top: (u16, u16),
    right_bottom: (u16, u16),
}

#[derive(Error, Debug, Deserialize, PartialEq, Eq)]
pub enum AsayakeError {
    #[error("Asayake is disconnected from komorebi.\n Is komorebi running?")]
    DisconnectFromKomorebi,
}

impl Default for AsayakeMonitorState {
    fn default() -> Self {
        AsayakeMonitorState {
            monitor_id: 0,
            focusing_workspace: 0,
            workspaces: vec![],
            size: Rect {
                left_top: (0, 0),
                right_bottom: (1920, 1080)
            }
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
