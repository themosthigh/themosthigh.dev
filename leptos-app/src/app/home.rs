use leptos::prelude::*;

use crate::constants::strings;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <a
            class="bg-red-700 hover:bg-red-800 transition-all group"
            href=strings::GH_URL
            target="_blank"
        >
            <div class="me-auto bg-yellow-100 h-px group-hover:w-full w-0 duration-300 delay-200"></div>

            <div class="max-w-7xl mx-auto px-8 py-16 ">
                <div class="font-bold grid gap-2">
                    <h3 class="text-white font-thin font-mono">goto;</h3>

                    // The big text
                    <h1 class="text-[calc(min(9rem,10vw))] text-yellow-100">
                        {strings::AKU}_{strings::GH_USERNAME}
                    </h1>

                    <h3 class="text-white font-mono font-thin opacity-30 group-hover:opacity-100 transition-opacity duration-600 delay-300">
                        {"// see more brainmade-made work ..."}
                    </h3>
                </div>
            </div>

            <div class="ms-auto bg-yellow-100 h-px group-hover:w-full w-0 transition-all duration-300 delay-300"></div>
        </a>
    }
}
