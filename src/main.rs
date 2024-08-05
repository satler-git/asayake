mod app;

use app::App;
use yew::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
