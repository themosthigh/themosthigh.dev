use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

// App modules
mod blog;
mod home;
mod projects;

use crate::components;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />

                // favicon
                <link rel="icon" type="image/svg+xml" href="favicon.svg" />

                // tailwind
                <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>

                // Google Fonts Prelude
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                // Space Mono
                <link
                    href="https://fonts.googleapis.com/css2?family=Space+Mono:ital,wght@0,400;0,700;1,400;1,700&display=swap"
                    rel="stylesheet"
                />
                // Space Grotesk
                <link
                    href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300..700&family=Space+Mono:ital,wght@0,400;0,700;1,400;1,700&display=swap"
                    rel="stylesheet"
                />

            </head>
            <body>
                <App />

                <style>
                    "html,
                    .font-sans {
                      font-family: \"Space Grotesk\", sans-serif;
                      font-size: 16px;
                    }
                    
                    .font-mono {
                      font-family: \"Space Mono\", sans-serif;
                      font-size: 16px;
                    }"
                </style>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-app.css" />

        // sets the document title
        <Title text="themosthigh" />

        <div class="flex flex-col min-h-screen bg-black text-white">

            // content for this welcome page
            <Router>

                // Top navigation bar
                <components::nav::MainNavigationBar />

                // Main content
                <main class="flex flex-col flex-1">
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=home::HomePage />
                        <Route path=StaticSegment("projects") view=projects::ProjectsPage />
                        <Route path=StaticSegment("blog") view=blog::BlogPage />
                    </Routes>
                </main>

                // Footer content
                <components::footer::Footer />
            </Router>
        </div>
    }
}
