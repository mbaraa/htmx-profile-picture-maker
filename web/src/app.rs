use crate::editor::moveable_image::MoveableImage;
use gloo::console;
use serde_json;
use yew::{classes, function_component, html, Callback, Html};

#[function_component(App)]
pub fn app() -> Html {
    let get_right_laser_rect = Callback::from(|rect| {
        console::log!("right");
        console::log!(serde_json::to_string(&rect).unwrap());
    });

    let get_left_laser_rect = Callback::from(|rect| {
        console::log!("left");
        console::log!(serde_json::to_string(&rect).unwrap());
    });

    html! {
        <main style="min-height: 100dvh" class={classes!("bg-gray")}>
            <MoveableImage
                start_x={0}
                start_y={0}
                image_path="/resources/laser-right.svg"
                title="Right Laser"
                aspect_ratio={0.6}
                width={171}
                get_rect={get_right_laser_rect}
            />
            <MoveableImage
                start_x={200}
                start_y={0}
                image_path="/resources/laser-left.svg"
                title="Left Laser"
                aspect_ratio={0.7}
                width={200}
                get_rect={get_left_laser_rect}
            />
            {"AAAAAAAAAAAAAAAAAAAAAAAA HELP!"}
        </main>
    }
}
