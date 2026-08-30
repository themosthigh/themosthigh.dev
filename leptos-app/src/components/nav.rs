use leptos::prelude::*;

use crate::constants::strings;

#[component]
pub fn MainNavigationBar() -> impl IntoView {
    view! {
        <nav class="py-4 px-8 h-[4rem] gap-4 sticky top-0 bg-black/90 backdrop-blur-xl">

            <div class="max-w-7xl mx-auto grid grid-cols-[1fr_auto_1fr] items-center">
                <a href="/" class="flex gap-2 items-center">
                    <span class="text-2xl">{strings::AKU}</span>
                </a>

                <div class="flex items-center justify-center gap-6">
                    <MainNavItem name="Home" href="/" />
                    <MainNavItem name="Blog" href="/blog" />
                    <MainNavItem name="Projects" href="/projects" />
                </div>

                <div class="flex justify-end items-center gap-6">
                    <MainNavItem name="Github" href=strings::GH_URL />
                </div>

            </div>
        </nav>
    }
}

#[component]
pub fn MainNavItem(name: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <a class="text-white/80 hover:underline text-xs uppercase" href=href>
            {name}
        </a>
    }
}
