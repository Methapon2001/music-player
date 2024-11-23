use std::fs::File;

use crate::playback::controls::MediaControls;
use crate::ui;

use iced::{
    time,
    widget::{
        button, column, container, horizontal_space, row, slider, stack, text, vertical_space,
    },
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
                    .seek(time::Duration::from_secs_f32(position))
                    .unwrap();
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // NOTE: update view every 500 millisecond when music is playing.
        if !self.controls.is_paused() && !self.controls.is_empty() {
            return time::every(time::Duration::from_millis(500)).map(|_| Message::Tick);
        }
        return Subscription::none();
    }

    pub fn view(&self) -> Element<Message> {
        let playback_info = &self.controls.playback_info;
        let playing = !self.controls.is_paused() && !self.controls.is_empty();

        let action = if playing {
            button("pause")
                .style(ui::button::danger)
                .on_press(Message::Pause)
        } else {
            button("play")
                .style(ui::button::success)
                .on_press(Message::Play)
        };

        let stats = row![
            horizontal_space(),
            text(if self.controls.is_empty() {
                format!("--:-- / --:--")
            } else {
                format!(
                    "{:02}:{:02} / {:02}:{:02}",
                    self.controls.get_pos().as_secs() / 60,
                    self.controls.get_pos().as_secs() % 60,
                    playback_info.total_duration.as_secs() / 60,
                    playback_info.total_duration.as_secs() % 60,
                )
            })
            .size(12),
            horizontal_space()
        ];

        let seek = slider(
            0.0..=playback_info.total_duration.as_secs_f32(),
            self.controls.get_pos().as_secs_f32(),
            Message::Seek,
        )
        .style(|theme, status| {
            let mut styled_slider = slider::default(theme, status);
            styled_slider.rail.border = styled_slider.rail.border.rounded(0);
            styled_slider
        });

        container(column![action, stats, seek].spacing(16))
            .padding(16)
            .into()
    }
}
