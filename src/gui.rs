mod gui_lib;
mod update;
mod view;

use view::view;
use update::update;

use iced::{
    window, Font, Theme, Settings,
};

fn main() -> iced::Result {
    let mut window_settings = window::Settings::default();
    window_settings.size = iced::Size {
        width: 1200.0,
        height: 780.0,
    };
    window_settings.min_size = Some(iced::Size {
        width: 1000.0,
        height: 660.0,
    });

    let mut app_settings = Settings::default();
    app_settings.default_font = Font {
        family: iced::font::Family::Monospace,
        ..Font::default()
    };
    app_settings.default_text_size = iced::Pixels(14.0);

    iced::application("bobs8085-gui", update, view)
        .theme(|_| Theme::Oxocarbon)
        .centered()
        .settings(app_settings)
        .window(window_settings)
        .run()
}
