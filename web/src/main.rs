mod app;
mod editor;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
