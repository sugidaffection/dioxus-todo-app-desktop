#![windows_subsystem = "windows"]

use dioxus::desktop::{Config, WindowBuilder};
mod app;
mod db;

use app::App;

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
