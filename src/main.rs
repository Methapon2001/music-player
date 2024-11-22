mod playback;
mod ui;
mod view;

use view::app::MediaPlayer;

fn main() -> iced::Result {
    iced::application(MediaPlayer::title, MediaPlayer::update, MediaPlayer::view)
        .subscription(MediaPlayer::subscription)
        .theme(MediaPlayer::theme)
        .window(iced::window::Settings {
            size: iced::Size::new(600.0, 400.0),
            min_size: Some(iced::Size::new(400.0, 300.0)),
            ..Default::default()
        })
        .run()
}
