use crate::editor::moveable_image::MoveableImage;
use yew::{classes, function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main style="min-height: 100dvh" class={classes!("bg-gray")}>
            <MoveableImage
                start_x={0}
                start_y={0}
                image_path="/resources/laser-right.svg"
                title="Right Laser"
                aspect_ratio={0.6}
                width={171}
            />
            <MoveableImage
                start_x={200}
                start_y={0}
                image_path="/resources/laser-left.svg"
                title="Left Laser"
                aspect_ratio={0.7}
                width={200}
            />
            {"AAAAAAAAAAAAAAAAAAAAAAAA HELP!"}
        </main>
    }
}
