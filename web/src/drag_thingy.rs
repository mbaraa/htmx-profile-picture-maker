use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
use web_sys::{window, EventTarget, HtmlDivElement, MouseEvent};
use yew::{
    classes, function_component, html, use_effect, use_node_ref, use_state, Html, Properties,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub start_x: u32,
    pub start_y: u32,
    pub image_path: String,
}

#[function_component(DragThingy)]
pub fn drag_thingy(props: &Props) -> Html {
    let document = window()
        .expect_throw("window is undefined")
        .document()
        .expect_throw("document is undefined");

    let offset = use_state(|| [0, 0]);
    let is_down = use_state(|| false);
    let overlay_ref = use_node_ref();

    {
        let div_ref = overlay_ref.clone();
        let offset = offset.clone();
        let is_down = is_down.clone();

        use_effect(move || {
            let div = div_ref
                .clone()
                .cast::<HtmlDivElement>()
                .expect("div_ref not attached to div element");

            let closure = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                is_down.set(true);
                let _target: Option<EventTarget> = e.target();
                offset.set([
                    div.offset_left() - e.client_x(),
                    div.offset_top() - e.client_y(),
                ]);
            });

            div_ref
                .clone()
                .cast::<HtmlDivElement>()
                .expect("div_ref not attached to div element")
                .add_event_listener_with_callback("pointerdown", closure.as_ref().unchecked_ref())
                .unwrap();

            move || {
                div_ref
                    .clone()
                    .cast::<HtmlDivElement>()
                    .expect("div_ref not attached to div element")
                    .remove_event_listener_with_callback(
                        "pointerdown",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        });
    };

    {
        let is_down = is_down.clone();
        let document = document.clone();

        use_effect(move || {
            let closure = Closure::<dyn FnMut(_)>::new(move |_e: MouseEvent| {
                is_down.set(false);
            });
            document
                .add_event_listener_with_callback("pointerup", closure.as_ref().unchecked_ref())
                .expect("AAAAAAAAAAAAAAAAAAA");
            move || {
                document
                    .remove_event_listener_with_callback(
                        "pointerup",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        });
    };

    {
        let offset = offset.clone();
        let is_down = is_down.clone();
        let _document = document.clone();

        use_effect(move || {
            let closure = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                e.prevent_default();
                let div = _document
                    .get_element_by_id("overlay")
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap();
                if *is_down {
                    div.style()
                        .set_property(
                            "left",
                            format!("{}px", e.client_x() + (*offset)[0])
                                .to_owned()
                                .as_str(),
                        )
                        .unwrap();
                    div.style()
                        .set_property(
                            "top",
                            format!("{}px", e.client_y() + (*offset)[1])
                                .to_owned()
                                .as_str(),
                        )
                        .unwrap();
                }
            });
            document
                .add_event_listener_with_callback("pointermove", closure.as_ref().unchecked_ref())
                .expect("AAAAAAAAAAAAAAAAAAA");
            move || {
                document
                    .remove_event_listener_with_callback(
                        "pointermove",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        });
    };

    html! {
        <div
            id="overlay"
            ref={overlay_ref}
            style={format!("
                position: absolute;
                top: {}px;
                left: {}px;
                background-color: green;
                padding: 50px;
                margin: 20px;
            ", props.start_y.clone(), props.start_x.clone())}>
                <div class={classes!("bg-[#112233]")}>{*is_down.clone()}</div>
        </div>
    }
}
