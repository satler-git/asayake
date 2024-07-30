#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use anyhow::{Context as _, Result};

use komorebi_client::{send_query, SocketMessage, State};
use winput::{
    message_loop::{self, EventReceiver},
    Action, Vk,
};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
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

#[component]
fn Home() -> Element {
    let receiver = use_signal(|| message_loop::start().unwrap());
    let mut alt = use_signal(|| alt_state(&*receiver.read()).unwrap());

    if *alt.read() {
        // let state = komorebi_state().unwrap();
    }
    rsx! {
        div {
            h1 { "High-Five counter: 0" }
            h1 { "Alt State: {alt}" }
            button { onclick: move |_| *alt.write() = alt_state(&*receiver.read()).unwrap() }
        }
    }
}
