use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Event, EventTarget, File, FileReader, HtmlInputElement};
use yew::{classes, function_component, html, use_state, Callback, Html};

#[function_component(PicturePicker)]
pub fn picture_picker() -> Html {
    let image_file = use_state(|| {
        File::new_with_u8_array_sequence(&JsValue::from(js_sys::Array::new()), "").unwrap()
    });
    let image_url = use_state(String::new);

    let pick_file = {
        let image_file = image_file.clone();
        let image_url = image_url.clone();

        // this little fucker, is a bumpy ride, or it's just skill issues, this is my second Yew
        // project, I'm not used to frontend being this serious!
        //
        // the thing is, I started with File::text(), which how I used to do it in JS, but promises
        // and futures are a bit tricky here, I spent like 2 hours trying to set the image's source
        // from a promise's callback, but got nothing, then I remembered the file reader, which,
        // well, as you can see, it's working!
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let file_target = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(file_target) = file_target {
                // this is needed somewhere in the future.
                image_file.set(file_target.files().unwrap().item(0).unwrap());

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
                })
                    as Box<dyn Fn(web_sys::ProgressEvent)>);

                fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                fr.read_as_data_url(&file_target.files().unwrap().item(0).unwrap())
                    .expect("blob not readable");
                onloadend_cb.forget();
            }
        })
    };

    html! {
        <div class={classes!("w-full", "grid", "grid-cols-1", "items-center", "place-items-center", "content-center")}>
            <img
              class={classes!("rounded-[10%]", "w-[350px]", "h-[350px]", "p-[15px]", "bg-gray-100")}
              id="image-to-upload"
              src={(*image_url).clone()}
              alt="Picked image"
            />
            <br />
            <input
              accept="image/*"
              class={classes!("hiddenn")}
              id="raised-button-file"
              type="file"
              size={5120}
              onchange={pick_file}
            />

            <label for="raised-button-file">
            {"mewo"}
            </label>

           // if errMsg.length > 0 {
           //   <label class={classes!("text-red-500", "text-[20px]")}>
           //     <br />
           //     {errMsg}
           //   </label>
           // }
        </div>
    }
}
