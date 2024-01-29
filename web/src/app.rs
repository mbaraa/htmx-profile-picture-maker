use crate::{
    editor::{
        moveable_image::{MoveableImage, Rect},
        picture_picker::{PicturePicker, Point},
    },
    footer::Footer,
    header::Header,
};
use gloo::{console, utils::document};
use gloo_net::http;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlElement;
use yew::{classes, function_component, html, use_state, Callback, Html};

#[derive(Serialize, Deserialize, Debug)]
struct RequestBody {
    pub pfp_b64: String,
    pub right_rect: Rect,
    pub left_rect: Rect,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseBody {
    pub htmx_pfp: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let error_msg = use_state(String::new);
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
    let image_position = use_state(|| Point { x: 0, y: 0 });

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
    let image_content = use_state(|| String::from("resources/non-htmx-pfp-placeholder.svg"));

    let set_image_content = {
        let image_content = image_content.clone();
        Callback::from(move |image_content_ref| {
            image_content.set(image_content_ref);
        })
    };
    let set_image_position = {
        let image_position = image_position.clone();
        Callback::from(move |point| {
            image_position.set(point);
        })
    };

    let do_something = {
        let image_content = image_content.clone();
        let right_rect = right_rect.clone();
        let left_rect = left_rect.clone();
        let error_msg = error_msg.clone();
        let image_position = image_position.clone();

        Callback::from(move |_| {
            let image_content = image_content.clone();
            let right_rect = right_rect.clone();
            let left_rect = left_rect.clone();

            if (*image_content).eq("resources/image-upload.png")
                || !(*image_content).contains("base64,")
            {
                error_msg.set("sEleCt aN iMaGe PLEASE!".to_string());
                return;
            }

            let error_msg = error_msg.clone();
            let image_position = image_position.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let req = http::Request::post("/api/generate-htmx-pfp")
                    .header("Content-Type", "application/json")
                    .body(
                        serde_json::to_string(&RequestBody {
                            pfp_b64: ((*image_content).clone()
                                [(*image_content).clone().find("base64,").unwrap() + 7..])
                                .to_string(),
                            right_rect: Rect {
                                x: (*right_rect).x - (*image_position).x as f64,
                                y: (*right_rect).y - (*image_position).y as f64,
                                height: (*right_rect).height,
                                width: (*right_rect).width,
                            },
                            left_rect: Rect {
                                x: (*left_rect).x - (*image_position).x as f64,
                                y: (*left_rect).y - (*image_position).y as f64,
                                height: (*left_rect).height,
                                width: (*left_rect).width,
                            },
                        })
                        .unwrap(),
                    );
                let resp = req.unwrap().send().await.unwrap();
                let resp_body: Result<ResponseBody, gloo_net::Error> = resp.json().await;

                match resp_body {
                    Ok(r) => {
                        let a = document().create_element("a").unwrap();
                        a.set_attribute(
                            "href",
                            format!("data:image/png;base64,{}", r.htmx_pfp).as_str(),
                        )
                        .unwrap();
                        a.set_attribute("download", "HTMXed pfp.png").unwrap();
                        let aaa = JsCast::dyn_into::<HtmlElement>(a).unwrap();
                        aaa.click();
                    }
                    Err(err) => {
                        error_msg.set(err.to_string());
                    }
                }
            });
            //console::log!((*image_content).clone());
            //console::log!(serde_json::to_string(&(*right_rect)).unwrap());
            //console::log!(serde_json::to_string(&(*left_rect)).unwrap());
        })
    };

    html! {
        <>
            <Header />
            <main style="min-height: 80dvh" class={classes!("bg-gray")}>
                <div class={classes!("w-full", "grid", "grid-cols-1", "content-center", "justify-items-center")}>
                    <PicturePicker
                        image_content={(*image_content).clone()}
                        set_image_content={set_image_content}
                        set_image_position={set_image_position}
                        max_file_size={7168}
                    />

                    // any possible error message
                    if (*error_msg).len() > 0 {
                      <label class={classes!("text-red-500", "text-[15px]")}>
                        <br />
                        {(*error_msg).clone()}
                      </label>
                    }

                    <button
                        class={classes!("p-[4px]", "px-[8px]", "bg-blue", "hover:bg-dark-blue", "text-dark-blue",
                                        "hover:text-blue", "rounded-[5px]", "w-[365px]")}
                        onclick={do_something}
                    >{"Generate HTMX Profile Picture"}</button>
                </div>

                <MoveableImage
                    start_x={0}
                    start_y={100}
                    image_path="/resources/laser-right.svg"
                    title="Right Laser"
                    aspect_ratio={0.542125}
                    width={171}
                    set_rect={set_right_laser_rect}
                />
                <MoveableImage
                    start_x={0}
                    start_y={460}
                    image_path="/resources/laser-left.svg"
                    title="Left Laser"
                    aspect_ratio={0.61144}
                    width={200}
                    set_rect={set_left_laser_rect}
                />
            </main>
            <Footer />
        </>
    }
}
