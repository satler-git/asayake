use anyhow::{Ok, Result};

use komorebi_client::{send_query, SocketMessage, State};
use winput::{message_loop, Action, Vk};

// TODO: GUIを作るときにaltはStateで管理(Pressedはイベントとしてずっとだから)
fn main() -> Result<()> {
    let receiver = message_loop::start().unwrap();

    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard { vk, action, .. } => match action {
                Action::Press => match vk {
                    Vk::Escape => {
                        if cfg!(debug_assertions) {
                            break;
                        }
                    }
                    Vk::Alt => println!("Alt pressed!"),
                    _ => {}
                },
                Action::Release => match vk {
                    Vk::Alt => {
                        println!("Alt released!");
                        let str_state = &send_query(&SocketMessage::State).unwrap();
                        let state: State = serde_json::from_str(str_state).unwrap();
                        println!("And git the State, Here {:?}", state);
                    }
                    _ => {}
                },
            },
            _ => (),
        }
    }
    Ok(())
}
