use leptos::prelude::*;

use crate::constants::strings;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="px-8 py-4">
            <div class="flex gap-2 justify-start max-w-7xl mx-auto item-center">
                <div class="flex-1 items-center flex">
                    <p class="text-sm opacity-50">{strings::GH_BIO}</p>
                </div>

                <a href=strings::BRAIN_MADE_WEBSITE target="_blank">
                    <img src=strings::BRAIN_MADE_SVG_WHITE class="w-16 h-16 opacity-50" />
                </a>
            </div>
        </footer>
    }
}
