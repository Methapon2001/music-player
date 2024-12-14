use std::fs::File;

use crate::playback::controls::MediaControls;
use crate::ui;

use iced::{
    time,
    widget::{
        button, column, container, horizontal_space, row, slider, stack, text, vertical_space,
    },
    Element, Subscription, Task, Theme,
};

use rfd::FileHandle;

#[derive(Debug, Clone)]
pub enum MediaPlayerMessage {
    Tick,

    Play,
    Pause,
    Stop,
    Seek(f32),
    Volume(f32),

    FileDialogOpen,
    FileDialogSelect(Option<FileHandle>),
}

#[derive(Default)]
pub struct MediaPlayer {
    controls: MediaControls,

    file_handle: Option<FileHandle>,
}

impl MediaPlayer {
    pub fn update(&mut self, message: MediaPlayerMessage) -> Task<MediaPlayerMessage> {
        match message {
            MediaPlayerMessage::Tick => Task::none(),

            // NOTE: controls music
            MediaPlayerMessage::Play => {
                if self.controls.is_empty() {
                    if let Some(file_handle) = &self.file_handle {
                        let file = File::open(file_handle.path()).unwrap();
                        self.controls.append(file).unwrap();
                    }
                }
                self.controls.play();
                Task::none()
            }
            MediaPlayerMessage::Pause => {
                self.controls.pause();
                Task::none()
            }
            MediaPlayerMessage::Stop => {
                self.controls.stop();
                Task::none()
            }
            MediaPlayerMessage::Seek(position) => {
                self.controls
                    .seek(time::Duration::from_secs_f32(position))
                    .unwrap();
                Task::none()
            }
            MediaPlayerMessage::Volume(volume) => {
                self.controls.set_volume(volume);
                Task::none()
            }

            // NOTE: update state
            MediaPlayerMessage::FileDialogOpen => Task::perform(
                async {
                    rfd::AsyncFileDialog::new()
                        .set_directory("/")
                        .pick_file()
                        .await
                },
                MediaPlayerMessage::FileDialogSelect,
            ),
            MediaPlayerMessage::FileDialogSelect(handle) => {
                self.file_handle = handle;
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<MediaPlayerMessage> {
        if !self.controls.is_paused() && !self.controls.is_empty() {
            // NOTE: update view every 500 millisecond when music is playing.
            return time::every(time::Duration::from_millis(100)).map(|_| MediaPlayerMessage::Tick);
        }
        return Subscription::none();
    }

    pub fn view(&self) -> Element<MediaPlayerMessage> {
        let playback_info = &self.controls.playback_info;
        let playing = !self.controls.is_paused() && !self.controls.is_empty();

        let mut play_button = button("play").style(ui::button::success);
        let mut pause_button = button("pause").style(ui::button::primary);
        let mut stop_button = button("stop").style(ui::button::danger);

        if self.controls.is_empty() && self.file_handle.is_some() {
            play_button = play_button.on_press(MediaPlayerMessage::Play);
        }
        if !self.controls.is_empty() {
            pause_button = pause_button.on_press(MediaPlayerMessage::Pause);
            stop_button = stop_button.on_press(MediaPlayerMessage::Stop);
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
            MediaPlayerMessage::Seek,
        )
        .style(|theme, status| {
            let mut styled_slider = slider::default(theme, status);
            styled_slider.rail.border = styled_slider.rail.border.rounded(0);
            styled_slider
        });

        let volume = slider(
            0.0..=1.5,
            self.controls.get_volume(),
            MediaPlayerMessage::Volume,
        )
        .step(0.1)
        .width(72);

        let gui = column![
            container(
                row![
                    button("Pick File")
                        .on_press(MediaPlayerMessage::FileDialogOpen)
                        .style(ui::button::primary),
                    text(
                        self.file_handle
                            .as_ref()
                            .map_or("No file selected", |handle| {
                                let path = handle.path();

                                if let Some(pathname) = path.to_str() {
                                    pathname
                                } else {
                                    "No file selected"
                                }
                            })
                    )
                ]
                .align_y(iced::Center)
                .spacing(8)
            )
            .padding(16)
            .height(iced::Length::Fill),
            stack![
                column![
                    vertical_space().height(10),
                    container(
                        row![
                            if playing { pause_button } else { play_button },
                            stop_button,
                            volume,
                            horizontal_space(),
                            stats.width(92)
                        ]
                        .align_y(iced::Center)
                        .spacing(8)
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
        ];

        gui.into()
    }
}
