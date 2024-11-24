use std::fs::File;

use crate::playback::controls::MediaControls;
use crate::ui;

use iced::{
    time,
    widget::{
        button, column, container, horizontal_space, row, slider, stack, text, text_input,
        vertical_space,
    },
    Element, Subscription, Theme,
};

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Play,
    Pause,
    Stop,
    Seek(f32),
    Volume(f32),
    Input(String),
}

#[derive(Default)]
pub struct MediaPlayer {
    controls: MediaControls,
    input: String,
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
                    let file = File::open(&self.input).unwrap();
                    self.controls.append(file).unwrap();
                }
                self.controls.play();
            }
            Message::Pause => {
                self.controls.pause();
            }
            Message::Stop => {
                self.controls.stop();
            }
            Message::Seek(position) => {
                self.controls
                    .seek(time::Duration::from_secs_f32(position))
                    .unwrap();
            }
            Message::Volume(volume) => {
                self.controls.set_volume(volume);
            }
            Message::Input(text) => {
                self.input = text;
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if !self.controls.is_paused() && !self.controls.is_empty() {
            // NOTE: update view every 500 millisecond when music is playing.
            return time::every(time::Duration::from_millis(500)).map(|_| Message::Tick);
        }
        return Subscription::none();
    }

    pub fn view(&self) -> Element<Message> {
        let playback_info = &self.controls.playback_info;
        let playing = !self.controls.is_paused() && !self.controls.is_empty();

        let mut play_button = button("play").style(ui::button::success);
        let mut pause_button = button("pause").style(ui::button::primary);
        let mut stop_button = button("stop").style(ui::button::danger);
        if !self.controls.is_empty() {
            play_button = play_button.on_press(Message::Play);
            pause_button = pause_button.on_press(Message::Pause);
            stop_button = stop_button.on_press(Message::Stop);
        }

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

        let volume = slider(0.0..=1.5, self.controls.get_volume(), Message::Volume)
            .step(0.1)
            .width(72);

        column![
            container(text_input("Music file...", &self.input).on_input(Message::Input))
                .padding(16)
                .height(iced::Length::Fill),
            stack![
                column![
                    vertical_space().height(10),
                    container(
                        row![
                            toggle_button,
                            stop_button,
                            volume,
                            horizontal_space(),
                            stats.width(92)
                        ]
                        .align_y(iced::Center)
                        .spacing(16)
                        .padding(16)
                    )
                    .width(iced::Length::Fill)
                    .style(|theme: &Theme| {
                        let palette = theme.extended_palette();

                        container::Style {
                            background: Some(iced::Background::from(
                                palette.background.strong.color.scale_alpha(0.03),
                            )),
                            ..Default::default()
                        }
                    })
                ],
                seek,
            ],
        ]
        .into()
    }
}
