use std::fs::File;

use crate::playback::controls::MediaControls;
use crate::ui;

use iced::widget::slider;
use iced::{
    time::Duration,
    widget::{button, column, container, horizontal_space, row, text},
    Element, Subscription, Theme,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    Tick,
    Play,
    Pause,
    Seek(f32),
}

#[derive(Default)]
pub struct MediaPlayer {
    controls: MediaControls,
}

impl MediaPlayer {
    pub fn title(&self) -> String {
        format!("Icy Player")
    }

    pub fn theme(&self) -> Theme {
        Theme::default()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {}
            Message::Play => {
                if self.controls.is_empty() {
                    let file = File::open("").unwrap();
                    self.controls.append(file).unwrap();
                } else {
                    self.controls.play();
                }
            }
            Message::Pause => {
                self.controls.pause();
            }
            Message::Seek(position) => {
                self.controls
                    .seek(Duration::from_secs_f32(position))
                    .unwrap();
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // NOTE: update view every 500 millisecond when music is playing.
        if !self.controls.is_paused() && !self.controls.is_empty() {
            return iced::time::every(Duration::from_millis(500)).map(|_| Message::Tick);
        }
        return Subscription::none();
    }

    pub fn view(&self) -> Element<Message> {
        let playback_info = &self.controls.playback_info;
        let playing = !self.controls.is_paused() && !self.controls.is_empty();

        let action = row![
            horizontal_space(),
            button(if playing { "pause" } else { "play" })
                .style(if playing {
                    ui::button::danger
                } else {
                    ui::button::primary
                })
                .on_press(if playing {
                    Message::Pause
                } else {
                    Message::Play
                }),
            horizontal_space()
        ];

        let stats = row![
            horizontal_space(),
            text(if self.controls.is_empty() {
                format!("Current: --:-- / --:--")
            } else {
                format!(
                    "Current - {:02}:{:02} / {:02}:{:02}",
                    self.controls.get_pos().as_secs() / 60,
                    self.controls.get_pos().as_secs() % 60,
                    playback_info.total_duration.as_secs() / 60,
                    playback_info.total_duration.as_secs() % 60,
                )
            }),
            horizontal_space()
        ];

        let seek = slider(
            0.0..=playback_info.total_duration.as_secs_f32(),
            self.controls.get_pos().as_secs_f32(),
            Message::Seek,
        );

        container(column![action, stats, seek].spacing(16))
            .padding(16)
            .into()
    }
}
