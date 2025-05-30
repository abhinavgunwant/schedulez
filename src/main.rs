//#![windows_subsystem = "windows"]
mod ui;
mod reader;
mod writer;
mod worker;
mod types;

use iced::{
    application, window::Settings, Result as IcedResult, Size, Theme
};

use ui::Window;

fn main() -> IcedResult {
    application("SchedulEZ", Window::update, Window::view)
        .window(Settings {
            size: Size { width: 640.0, height: 480.0 },
            resizable: false,
            transparent: true,
            ..Settings::default()
        })
        .theme(|_| Theme::Dark)
        .run()
}

