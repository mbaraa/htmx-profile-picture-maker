use gloo::console;
use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
use web_sys::{window, EventTarget, HtmlDivElement, MouseEvent};
use yew::{
    classes, function_component, html, use_effect, use_node_ref, use_state, Html, Properties,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub start_x: u32,
    pub start_y: u32,
    pub title: String,
    pub image_path: String,
}

#[function_component(DragThingy)]
pub fn drag_thingy(props: &Props) -> Html {
    let document = window()
        .expect_throw("window is undefined")
        .document()
        .expect_throw("document is undefined");

    let offset = use_state(|| [0, 0]);
    let is_move = use_state(|| false);
    let overlay_ref = use_node_ref();
    let overlay_id = use_state(|| format!("overlay{}", props.image_path.clone()));

    let is_resize = use_state(|| false);
    let image_ref = use_node_ref();

    let remove_event_listener_with_callback = {};

    // move the whole thing
    {
        let div_ref = overlay_ref.clone();
        let offset = offset.clone();
        let is_move = is_move.clone();
        use_effect(move || {
            let div = div_ref
                .clone()
                .cast::<HtmlDivElement>()
                .expect("div_ref not attached to div element");
            let closure = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                is_move.set(true);
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
        let is_move = is_move.clone();
        let document = document.clone();
        use_effect(move || {
            let closure = Closure::<dyn FnMut(_)>::new(move |_e: MouseEvent| {
                is_move.set(false);
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
        let is_move = is_move.clone();
        let _document = document.clone();
        let overlay_id = overlay_id.clone();
        use_effect(move || {
            let closure = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                e.prevent_default();
                let div = _document
                    .get_element_by_id(overlay_id.to_owned().as_str())
                    .unwrap()
                    .dyn_into::<HtmlDivElement>()
                    .unwrap();
                console::log!(e.type_());
                if *is_move {
                    console::log!(div.client_width());
                    console::log!(div.client_height());
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

    // resize the bloody image
    {
        //       let is_resize = is_resize.clone();
        //       use_effect(move || {
        //           let closure = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
        //               is_resize.set(true);
        //           });
        //           document
        //               .add_event_listener_with_callback("pointerdown", closure.as_ref().unchecked_ref())
        //               .unwrap();
        //
        //           move || {
        //               document
        //                   .remove_event_listener_with_callback(
        //                       "pointerdown",
        //                       closure.as_ref().unchecked_ref(),
        //                   )
        //                   .unwrap();
        //           }
        //       });
    };

    {
        //       let is_resize = is_move.clone();
        //       let document = document.clone();
        //       use_effect(move || {
        //           let closure = Closure::<dyn FnMut(_)>::new(move |_e: MouseEvent| {
        //               is_resize.set(false);
        //           });
        //           document
        //               .add_event_listener_with_callback("pointerup", closure.as_ref().unchecked_ref())
        //               .expect("AAAAAAAAAAAAAAAAAAA");
        //
        //           move || {
        //               document
        //                   .remove_event_listener_with_callback(
        //                       "pointerup",
        //                       closure.as_ref().unchecked_ref(),
        //                   )
        //                   .unwrap();
        //           }
        //       });
    };

    html! {
        <div
            id={format!("overlay{}", props.image_path.clone())}
            ref={overlay_ref}
            style={format!("
                position: absolute;
                top: {}px;
                left: {}px;
            ", props.start_y.clone(), props.start_x.clone())}>
                <div
                    ref={image_ref}
                    class={classes!("bg-clip-border", "bg-center", "bg-no-repeat", "bg-cover", "resize", "overflow-auto", "w-[120px]", "h-[230px]", "max-w-[800px]", "max-h-[1200px]")}
                    style={format!("
                        background-image: url('{}');
                        aspect-ratio: 11 / 22;
                    ", props.image_path.clone())}
                ></div>
                <div class={classes!("bg-blue", "hover:cursor-move", "flex", "justify-between", "p-[5px]", "rounded-[2px]")}>
                    <p class={classes!("text-dark-blue", "font-medium")}>{props.title.clone()}</p>
                    <div class={classes!("flex", "gap-x-[2.5px]")}>
                        <div
                            class={classes!("bg-[url('/resources/cursor-move.svg')]", "bg-center", "w-[25px]", "h-[25px]")}>
                        </div>
                        <div
                            class={classes!("bg-[url('/resources/cursor-nwse-resize.svg')]", "bg-center", "w-[25px]", "h-[25px]", "hover:cursor-nwse-resize")}>
                        </div>
                    </div>
                </div>
        </div>
    }
}
