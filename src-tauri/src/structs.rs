use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct AsayakeMonitorState {
    monitor_index: usize,
    focusing_workspace: usize,
    workspaces: Vec<WorkspaceForSend>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
struct WorkspaceForSend {}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
enum WorkspaceItem {
    Window(WindowForSend),
    WindowStack(Vec<WindowForSend>)
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