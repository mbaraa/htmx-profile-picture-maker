use gloo::console;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Event, EventTarget, File, FileReader, HtmlDivElement, HtmlInputElement};
use yew::{
    classes, function_component, html, use_effect_with, use_node_ref, use_state, Callback, Html,
    Properties,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    /// file size in kilobytes
    pub max_file_size: u64,
    pub image_content: String,
    pub set_image_content: Callback<String>,
    pub set_image_position: Callback<Point>,
}

#[function_component(PicturePicker)]
pub fn picture_picker(props: &Props) -> Html {
    let image_content = use_state(|| props.image_content.clone());
    let set_image_content = use_state(|| props.set_image_content.clone());
    let set_image_position = use_state(|| props.set_image_position.clone());

    let div_ref = use_node_ref();
    let display_file_name = use_state(|| String::from("Select a file"));
    let image_file = use_state(|| {
        File::new_with_u8_array_sequence(&JsValue::from(js_sys::Array::new()), "").unwrap()
    });
    let max_file_size = use_state(|| props.max_file_size.clone() as f64);
    let error_msg = use_state(String::new);

    let pick_file = {
        let image_file = image_file.clone();
        let div_ref = div_ref.clone();
        let set_image_position = set_image_position.clone();
        let display_file_name = display_file_name.clone();

        Callback::from(move |e: Event| {
            // get the image's position to set the lasers' offsets
            let div_rect = div_ref
                .clone()
                .cast::<HtmlDivElement>()
                .expect("div_ref not attached to div element")
                .get_bounding_client_rect();
            set_image_position.emit(Point {
                x: div_rect.x() as u32,
                y: div_rect.y() as u32,
            });

            let target: Option<EventTarget> = e.target();
            // dyn_into needs JsCast, at least here ig.
            let file_target = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(file_target) = file_target {
                image_file.set(file_target.files().unwrap().item(0).unwrap());
                let file_name_raw = file_target.files().unwrap().item(0).unwrap().name();
                let file_name = if file_name_raw.len() > 20 {
                    format!(
                        "{}...{}",
                        file_name_raw[0..13].to_string(),
                        file_name_raw[file_name_raw.len() - 7..].to_string()
                    )
                } else {
                    file_name_raw
                };
                display_file_name.set(format!("Selected: {}", file_name));
            }
        })
    };

    {
        let image_content = image_content.clone();
        let image_file_c = image_file.clone();
        let error_msg = error_msg.clone();
        let max_file_size = max_file_size.clone();
        let set_image_content = set_image_content.clone();

        use_effect_with(image_file, move |_| {
            // initial state of the image file.
            if (*image_file_c).name().len() == 0 {
                return;
            }
            if (*image_file_c).size() > (*max_file_size) * 1000. {
                error_msg.set(String::from(format!(
                    "{}KB is the maximum allowed file size!",
                    *max_file_size
                )));
                return;
            }
            if !(*image_file_c).type_().contains("image/png") {
                error_msg.set(String::from(
                    "I'm guessing things got personal, 'cuz this doesn't look like a png to me...",
                ));
                return;
            }

            // this little fucker, is a bumpy ride, or it's just skill issues, this is my second Yew
            // project, I'm not used to frontend being this serious!
            //
            // the thing is, I started with File::text(), which how I used to do it in JS, but promises
            // and futures are a bit tricky here, I spent like 2 hours trying to set the image's source
            // from a promise's callback, but got nothing, then I remembered the file reader, which,
            // well, as you can see, it's working!
            //
            // fr fr fr, no jk, the naming was not intentional I swear!
            let fr = FileReader::new().unwrap();
            let fr_c = fr.clone();
            // clone the image url, to move it to another function, bla bla bla.
            let image_content = image_content.clone();

            // create onLoadEnd callback, where this will read the file and just slap its
            // content to the img element on the DOM.
            let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
                // sine I'm only calling `read_as_data_url`, it will read the image as a base64
                // string, and well, here I am.
                image_content.set(fr_c.result().unwrap().to_owned().as_string().unwrap());
                set_image_content.emit(fr_c.result().unwrap().to_owned().as_string().unwrap());
            }) as Box<dyn Fn(web_sys::ProgressEvent)>);

            fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            // &(*var) means the same thing right?
            // well, no, first I have to get the value from the state,
            // then cast the `web_sys::File` to a `web_sys::Blob`.
            fr.read_as_data_url(&(*image_file_c))
                .expect("blob not readable");
            onloadend_cb.forget();
        });
    };

    html! {
        <div
            ref={div_ref}
            class={classes!("w-fit", "grid", "grid-cols-1")}>
            <img
              class={classes!("rounded-[16px]", "w-[365px]", "h-[365px]", "bg-gray-100")}
              id="image-to-upload"
              src={(*image_content).clone()}
              alt="Picked image"
              title="Select a profile picture to htmx it up!"
            />
            <br />
            <input
              accept="image/png"
              class={classes!("hidden")}
              id="raised-button-file"
              type="file"
              size={(*max_file_size).to_string()}
              onchange={pick_file}
            />

            if (*error_msg).len() > 0 {
              <label class={classes!("text-red-500", "text-[15px]")}>
                <br />
                {(*error_msg).clone()}
              </label>
            }

            <label
                for="raised-button-file"
                class={classes!("p-[3px]", "px-[6px]", "bg-blue", "hover:bg-dark-blue", "text-dark-blue",
                                "hover:text-blue", "rounded-[5px]", "cursor-pointer", "my-[10px]")}
            >
                {(*display_file_name).clone()}
            </label>

        </div>
    }
}
