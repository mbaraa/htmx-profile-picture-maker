use crate::drag_thingy::DragThingy;
use yew::{classes, function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main style="min-height: 100dvh" class={classes!("bg-gray")}>
            <DragThingy start_x={0} start_y={0} image_path="/resources/laser-right.svg" title="Right Laser"/>
            <DragThingy start_x={300} start_y={0} image_path="/resources/laser-left.svg" title="Left Laser"/>
            {"AAAAAAAAAAAAAAAAAAAAAAAA HELP!"}
        </main>
    }
}
