mod playback;
mod ui;
mod view;

use iced::{
    widget::{button, container},
    Element, Subscription, Task, Theme,
};
use view::player::{MediaPlayer, MediaPlayerMessage};

#[derive(Default)]
enum View {
    #[default]
    App,
    MediaPlayer(MediaPlayer),
}

#[derive(Debug, Clone)]
enum Message {
    MediaPlayerView,
    MediaPlayerMessage(MediaPlayerMessage),
}

#[derive(Default)]
struct Application {
    view: View,
}

impl Application {
    pub fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }

    pub fn title(&self) -> String {
        String::from("Rusty Player")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MediaPlayerView => {
                self.view = View::MediaPlayer(MediaPlayer::default());
                Task::none()
            }
            Message::MediaPlayerMessage(message) => {
                if let View::MediaPlayer(view) = &mut self.view {
                    view.update(message).map(Message::MediaPlayerMessage)
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match &self.view {
            View::App => container(
                button("Player")
                    .style(ui::button::primary)
                    .on_press(Message::MediaPlayerView),
            )
            .padding(10)
            .into(),
            View::MediaPlayer(view) => view.view().map(Message::MediaPlayerMessage),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match &self.view {
            View::App => Subscription::none(),
            View::MediaPlayer(view) => view.subscription().map(Message::MediaPlayerMessage),
        }
    }
}

fn main() -> iced::Result {
    iced::application(Application::title, Application::update, Application::view)
        .subscription(Application::subscription)
        .theme(Application::theme)
        .window(iced::window::Settings {
            size: iced::Size::new(600.0, 400.0),
            min_size: Some(iced::Size::new(400.0, 300.0)),
            ..Default::default()
        })
        .run()
}
