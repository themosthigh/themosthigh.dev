use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};

use crate::constants::projects;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    provide_meta_context();
    view! {
        <head>
            <Title text="Projects" />
        </head>

        <div class="px-8 py-16">
            <div class="max-w-7xl mx-auto ">
                <div class="font-bold grid gap-16">
                    <h1 class="text-4xl md:text-6xl text-yellow-100">Projects</h1>

                    <div class="flex flex-col gap-0">

                        {projects::get_projects()
                            .into_iter()
                            .map(|project| {
                                view! {
                                    <ProjectCard
                                        name=project.name.to_string()
                                        description=project.description.to_string()
                                        github_url=project.github_url.to_string()
                                    />
                                }
                            })
                            .collect::<Vec<_>>()}

                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProjectCard(name: String, description: String, github_url: String) -> impl IntoView {
    view! {
        <a
            href=github_url
            target="_blank"
            class="flex flex-col gap-2 group py-4 not-last:border-b border-yellow-100/20 hover:bg-white/5
            "
        >
            <h2 class="text-lg font-bold group-hover:underline">{name}</h2>
            <p class="text-xs opacity-50 font-thin">{description}</p>
        </a>
    }
}
