mod app;
mod editor;
mod footer;
mod header;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
