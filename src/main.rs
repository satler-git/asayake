use anyhow::{Context as _, Result};

use komorebi_client::{send_query, SocketMessage, State};
use winput::{
    message_loop::{self, EventReceiver},
    Action, Vk,
};

fn main() {
    
}

fn alt_state(receiver: &EventReceiver) -> Result<bool> {
    match receiver.next_event() {
        message_loop::Event::Keyboard {
            vk: Vk::Alt,
            action,
            ..
        } => match action {
            Action::Press => Ok(true),
            Action::Release => Ok(false),
        },
        _ => Ok(false),
    }
}

fn komorebi_state() -> Result<State> {
    sonic_rs::from_str(&send_query(&SocketMessage::State)?).context("Unable to get the state of komorebi now.")
    // If you get this error.
    // You may be running a different versionof komorebi (We're using komorebi-client v0.1.28)
    // or, you may not running komorebi
}
