use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Todo List App"),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Home {}

    }
}

#[derive(Clone, PartialEq)]
pub struct Task {
    title: String,
    tag: String,
    status: String,
}

#[component]
pub fn Home() -> Element {
    let tasks = use_signal::<Vec<Task>>(|| vec![]);
    rsx! {
        div {
            class: "bg-gray-950 text-white min-h-screen p-6 font-sans",
            div {
                class: "max-w-4xl mx-auto",
                h1 {
                    class: "text-3xl font-bold mb-6 text-center text-red-500",
                    "My Todo List"
                },
                Input { tasks: tasks.clone() },
                TaskList { tasks }
            }

        }
    }
}

#[component]
pub fn Input(tasks: Signal<Vec<Task>>) -> Element {
    let mut new_title = use_signal(|| String::new());
    let mut new_tag = use_signal(|| "gameplay".to_string());

    let add_task = move |_| {
        if new_title.read().trim().is_empty() {
            return;
        }

        tasks.write().push(Task {
            title: new_title.read().clone(),
            tag: new_tag.read().clone(),
            status: "Pending".to_string(),
        });

        new_title.set("".into());
    };

    rsx! {
        div {
            class: "flex flex-col md:flex-row gap-4 mb-8",
            input {
                class: "flex-1 px-4 py-2 rounded-lg bg-gray-800 border border-gray-700 focus:outline-none focus:ring-2 focus:ring-red-500",
                placeholder: "New Task",
                oninput: move |e| new_title.set(e.value().clone()),
            },
            select {
                class: "pl-4 pr-12 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white appearance-none focus:outline-none focus:ring-2 focus:ring-red-500",
                onchange: move |e: Event<FormData>| {
                    new_tag.set(e.value());
                },
                option {
                    value: "gameplay",
                    "🎮 Gameplay"
                },
                option {
                    value: "guide",
                    "📘 Guide"
                },
                option {
                    value: "shorts",
                    "🎬 Shorts"
                }
            },
            button {
                class: "px-6 py-2 bg-red-600 hover:bg-red-700 rounded-lg text-white font-semibold",
                onclick: add_task,
                "Add Task",
            }
        }
    }
}

#[component]
pub fn TaskList(tasks: Signal<Vec<Task>>) -> Element {
    rsx! {
            div {
                class: "space-y-4",
                {tasks.read().iter().enumerate().map(|(idx, task)| rsx! {
                    div {
                        key: "{idx}",
                        class: "flex items-start justify-between p-4 bg-gray-900 rounded-lg border border-gray-700",
                        div {
                            h3 { class: "text-lg font-semibold", "{task.title}" }
                            p {
                                class: "text-sm text-gray-400",
                                "Tag: ",
                                span { class: "text-yellow-400", "{task.tag}" }
                            }
                        },
                        div {
                            class: "flex items-center gap-4",
                            select {
                                class: "pl-4 pr-12 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white appearance-none focus:outline-none focus:ring-2 focus:ring-red-500",
                                option { selected: task.status == "Pending", "Pending" }
                                option { selected: task.status == "In Progress", "In Progress" }
                                option { selected: task.status == "Uploaded", "Uploaded" }
                            },
                            button {
                                class: "text-red-400 hover:text-red-600 text-sm",
                                onclick: move |_| {
                                    tasks.write().remove(idx);
                                },
                                "✕"
                            }
                        }
                    }
                })
            }
        }
    }
}
