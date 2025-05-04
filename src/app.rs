use crate::ui;
use crate::view::player::{MediaPlayer, MediaPlayerMessage};

use iced::{
    widget::{button, container},
    Element, Subscription, Task, Theme,
};

#[derive(Default)]
enum View {
    #[default]
    App,
    MediaPlayer(MediaPlayer),
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    MediaPlayerView,
    MediaPlayerMessage(MediaPlayerMessage),
}

#[derive(Default)]
pub struct App {
    view: View,
}

impl App {
    pub fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }

    pub fn title(&self) -> String {
        String::from("Rusty Player")
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::MediaPlayerView => {
                self.view = View::MediaPlayer(MediaPlayer::default());
                Task::none()
            }
            AppMessage::MediaPlayerMessage(message) => {
                if let View::MediaPlayer(view) = &mut self.view {
                    view.update(message).map(AppMessage::MediaPlayerMessage)
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self) -> Element<AppMessage> {
        match &self.view {
            View::App => container(
                button("Player")
                    .style(ui::button::primary)
                    .on_press(AppMessage::MediaPlayerView),
            )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
            .padding(10)
            .into(),
            View::MediaPlayer(view) => view.view().map(AppMessage::MediaPlayerMessage),
        }
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        match &self.view {
            View::App => Subscription::none(),
            View::MediaPlayer(view) => view.subscription().map(AppMessage::MediaPlayerMessage),
        }
    }
}
