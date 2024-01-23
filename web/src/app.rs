use crate::drag_thingy::DragThingy;
use yew::{classes, function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main style="min-height: 100dvh" class={classes!("bg-gray")}>
            <DragThingy start_x={0} start_y={0} image_path="/resources/laser-right.svg" title="Right Laser" aspect_ratio={0.6} width={171} />
            <DragThingy start_x={220} start_y={0} image_path="/resources/laser-left.svg" title="Left Laser" aspect_ratio={0.7} width={200} />
            {"AAAAAAAAAAAAAAAAAAAAAAAA HELP!"}
        </main>
    }
}
