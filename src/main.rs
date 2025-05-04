mod app;
mod playback;
mod ui;
mod view;

use app::App;

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .window(iced::window::Settings {
            size: iced::Size::new(600.0, 400.0),
            min_size: Some(iced::Size::new(400.0, 300.0)),
            ..Default::default()
        })
        .run()
}
