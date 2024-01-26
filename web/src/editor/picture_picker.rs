use gloo::console;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Event, EventTarget, File, FileReader, HtmlInputElement};
use yew::{
    classes, function_component, html, use_effect_with, use_state, Callback, Html, Properties,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    /// file size in kilobytes
    pub max_file_size: u64,
    pub image_file: File,
    pub set_image_file: Callback<File>,
}

#[function_component(PicturePicker)]
pub fn picture_picker(props: &Props) -> Html {
    let image_file = use_state(|| props.image_file.clone());
    let set_image_file = use_state(|| props.set_image_file.clone());

    let max_file_size = use_state(|| props.max_file_size.clone() as f64);
    let image_url = use_state(|| String::from("resources/image-upload.png"));
    let error_msg = use_state(String::new);

    let pick_file = {
        let image_file = image_file.clone();
        let set_image_file = set_image_file.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            // dyn_into needs JsCast, at least here ig.
            let file_target = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(file_target) = file_target {
                image_file.set(file_target.files().unwrap().item(0).unwrap());
                set_image_file.emit(file_target.files().unwrap().item(0).unwrap());
            }
        })
    };

    {
        let image_url = image_url.clone();
        let image_file_c = image_file.clone();
        let error_msg = error_msg.clone();
        let max_file_size = max_file_size.clone();

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
            if !(*image_file_c).type_().contains("image") {
                error_msg.set(String::from(
                    "I'm guessing things got personal, 'cuz this doesn't look like a file to me...",
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
            let image_url = image_url.clone();

            // create onLoadEnd callback, where this will read the file and just slap its
            // content to the img element on the DOM.
            let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
                // sine I'm only calling `read_as_data_url`, it will read the image as a base64
                // string, and well, here I am.
                image_url.set(fr_c.result().unwrap().to_owned().as_string().unwrap());
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
        <div class={classes!("w-fit", "grid", "grid-cols-1")}>
            <img
              class={classes!("rounded-[16px]", "min-w-[365px]", "min-h-[365px]", "max-w-[600px]", "max-h-[600px]", "p-[15px]", "bg-gray-100")}
              id="image-to-upload"
              src={(*image_url).clone()}
              alt="Picked image"
              title="Select a profile picture to htmx it up!"
            />
            <br />
            <input
              accept="image/*"
              class={classes!("hidden")}
              id="raised-button-file"
              type="file"
              size={5120}
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
                {"Select file"}
            </label>

        </div>
    }
}
