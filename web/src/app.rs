use crate::editor::{moveable_image::MoveableImage, picture_picker::PicturePicker};
use gloo::console;
use serde_json;
use wasm_bindgen::JsValue;
use web_sys::File;
use yew::{classes, function_component, html, use_effect_with, use_state, Callback, Html};

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
    let image_file = use_state(|| {
        File::new_with_u8_array_sequence(&JsValue::from(js_sys::Array::new()), "").unwrap()
    });

    let set_image_file = {
        let image_file = image_file.clone();
        Callback::from(move |image_file_ref| {
            image_file.set(image_file_ref);
        })
    };

    {
        let image_file = image_file.clone();
        let image_file_c = image_file.clone();
        use_effect_with(image_file, move |_| {
            console::log!((*image_file_c).name());
        });
    };

    let do_something = {
        let image_file = image_file.clone();
        Callback::from(move |_| {
            console::log!((*image_file).name());
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
                get_rect={get_right_laser_rect}
            />
            <MoveableImage
                start_x={200}
                start_y={460}
                image_path="/resources/laser-left.svg"
                title="Left Laser"
                aspect_ratio={0.7}
                width={200}
                get_rect={get_left_laser_rect}
            />
            <PicturePicker
                image_file={(*image_file).clone()}
                set_image_file={set_image_file}
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
