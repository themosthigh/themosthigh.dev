use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};

#[component]
pub fn BlogPage() -> impl IntoView {
    provide_meta_context();
    view! {
        <head>
            <Title text="Blog" />
        </head>

        <div class="px-8 py-16">
            <div class="max-w-7xl mx-auto ">
                <div class="font-bold grid gap-16">
                    <h1 class="text-4xl md:text-6xl text-yellow-100">Blog</h1>
                    <p class="text-xs opacity-50 font-thin font-mono">
                        {"// TODO: I'll keep you posted"}
                    </p>
                </div>
            </div>
        </div>
    }
}
