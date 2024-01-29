use yew::{classes, function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class={classes!("h-[300px]", "bg-dark-blue", "text-white")}>
            <div class={classes!("h-[300px]", "flex", "justify-between", "items-center", "px-[30px]", "md:px-[35vw]")}>
                <div>
                    <h3 class={classes!("text-xl")}>{"haiku"}</h3>
                    <div class={classes!("italic")}>
                        <p>{"javascript fatigue:"}</p>
                        <p>{"longing for a hypertext"}</p>
                        <p>{"already in hand"}</p>
                    </div>
                </div>
                <div>
                    <p>{"This project is not affiliated with htmx, "}<i>{"YET!"}</i></p>
                    <p>{"And it's free and open source,"}</p>
                    <p>{"You can frok it and whatever on GitHub,"}</p>
                    <a
                        class={classes!("underline", "hover:text-blue")}
                        href="https://github.com/mbaraa/htmx-profile-picture-maker"
                    >
                        {"From here"}
                    </a>
                </div>
            </div>
            <p class={classes!("text-xl", "flex", "justify-center", "w-full", "text-center", "bg-dark-blue", "pb-[30px]")}>
                <span class={classes!("flex", "items-center")}>
                    {"Made with 🧉 by "}
                    <a
                        class={classes!("inline-block")}
                        title="Baraa Al-Masri"
                        href="https://mbaraa.com"
                    >
                        <img
                            class={classes!("w-[30px]", "h-[30px]", "ms-[5px]")}
                            height="30"
                            width="30"
                            src="https://mbaraa.com/resources/images/favicon.png"
                        />
                    </a>
                </span>
            </p>
        </footer>
    }
}
