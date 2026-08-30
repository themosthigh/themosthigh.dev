pub mod strings {
    pub const AKU: &str = "惡";
    pub const GH_USERNAME: &str = "themosthigh";
    pub const GH_BIO: &str = "The struggle for justice is an ongoing and necessary pursuit that should prevail over laws and institutions.";
    pub const GH_URL: &str = "https://github.com/themosthigh";

    pub const BRAIN_MADE_WEBSITE: &str = "https://brainmade.org";
    pub const BRAIN_MADE_SVG_WHITE: &str = "https://brainmade.org/white-logo.svg";
}

pub mod projects {
    pub struct Project {
        pub name: &'static str,
        pub description: &'static str,
        pub github_url: &'static str,
    }

    pub fn get_projects() -> Vec<Project> {
        vec![
            Project {
                name: "hermes",
                description: "A native HTTP client written in Rust and GTK.",
                github_url: "https://github.com/themosthigh/hermes",
            },
            Project {
                name: "din",
                description: "A quran reader app, Qibla finder, and more for Android, iOS, Linux ... Flutter baby",
                github_url: "https://github.com/themosthigh/din_flytter",
            },
            Project {
                name: "themosthigh",
                description: "A simple static website written in Rust using the Leptos framework.",
                github_url: "https://github.com/themosthigh/themosthigh.github.io",
            },
            Project {
                name: "darcula.nvim",
                description: "My darcula Neovim theme",
                github_url: "https://github.com/themosthigh/darcula.nvim",
            },
        ]
    }
}
