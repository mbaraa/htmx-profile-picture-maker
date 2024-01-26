use crate::editor::{
    moveable_image::{MoveableImage, Rect},
    picture_picker::PicturePicker,
};
use gloo::console;
use serde_json;
use wasm_bindgen::JsValue;
use web_sys::File;
use yew::{classes, function_component, html, use_effect_with, use_state, Callback, Html};

#[function_component(App)]
pub fn app() -> Html {
    let left_rect = use_state(|| Rect {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    });
    let right_rect = use_state(|| Rect {
        x: 0.,
        y: 0.,
        width: 0.,
        height: 0.,
    });

    let set_right_laser_rect = {
        let right_rect = right_rect.clone();
        Callback::from(move |rect| {
            right_rect.set(rect);
        })
    };

    let set_left_laser_rect = {
        let left_rect = left_rect.clone();
        Callback::from(move |rect| {
            left_rect.set(rect);
        })
    };
    let image_content = use_state(|| String::from("resources/image-upload.png"));

    let set_image_content = {
        let image_content = image_content.clone();
        Callback::from(move |image_content_ref| {
            image_content.set(image_content_ref);
        })
    };

    {
        let image_content = image_content.clone();
        let image_content_c = image_content.clone();
        use_effect_with(image_content, move |_| {
            console::log!((*image_content_c).clone());
        });
    };

    let do_something = {
        let image_content = image_content.clone();
        let right_rect = right_rect.clone();
        let left_rect = left_rect.clone();
        Callback::from(move |_| {
            console::log!((*image_content).clone());
            console::log!(serde_json::to_string(&(*right_rect)).unwrap());
            console::log!(serde_json::to_string(&(*left_rect)).unwrap());
        })
    };

    html! {
        <main style="min-height: 100dvh" class={classes!("bg-gray")}>
            <MoveableImage
                start_x={0}
                start_y={460}
                image_path="/resources/laser-right.svg"
                title="Right Laser"
                aspect_ratio={0.6}
                width={171}
                set_rect={set_right_laser_rect}
            />
            <MoveableImage
                start_x={200}
                start_y={460}
                image_path="/resources/laser-left.svg"
                title="Left Laser"
                aspect_ratio={0.7}
                width={200}
                set_rect={set_left_laser_rect}
            />
            <PicturePicker
                image_content={(*image_content).clone()}
                set_image_content={set_image_content}
                max_file_size={7168}
            />
            <button
                class={classes!("p-[3px]", "px-[6px]", "bg-blue", "hover:bg-dark-blue", "text-dark-blue",
                                "hover:text-blue", "rounded-[5px]")}
                onclick={do_something}
            >{"Something"}</button>
        </main>
    }
}
