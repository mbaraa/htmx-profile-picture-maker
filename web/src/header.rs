use yew::{classes, function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header
            class={classes!("h-[95px]", "mb-[5px]", "px-[50px]", "border-b", "border-b-white", "flex", "justify-center", "justify-between", "items-center",)}
        >
           <a href="/" class={classes!("flex", "justify-center", "items-center")}>
             <img
                 class={classes!("w-[80px]", "h-[80px]")}
                 width="80"
                 height="80"
                 src="resources/favicon.png"
             />
             <h1 class="hidden md:block ms-[25px] text-[#EDEDED] text-[24px] font-bold">
               {"HTMX Profile Picture Maker"}
             </h1>
           </a>
           <a
             class="github-button"
             href="https://github.com/mbaraa/htmx-profile-picture-maker"
             data-color-scheme="no-preference: dark_dimmed; light: light; dark: dark;"
             data-size="large"
             data-show-count="true"
             aria-label="Star mbaraa/github-graph-drawer on GitHub"
             >{"Star"}</a
           >
        </header>
    }
}
