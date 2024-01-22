mod app;
mod drag_thingy;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
