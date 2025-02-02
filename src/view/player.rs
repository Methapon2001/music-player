use std::fs::File;

use crate::playback::controls::MediaControls;
use crate::ui;

use iced::{
    time,
    widget::{
        button, column, container, horizontal_space, image, row, slider, stack, text,
        vertical_space,
    },
    Element, Subscription, Task, Theme,
};
use lofty::file::TaggedFileExt;
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
    FileDialogHandle(Option<FileHandle>),

    ImageHandle(Option<image::Handle>),
}

#[derive(Default)]
pub struct MediaPlayer {
    controls: MediaControls,
    file_handle: Option<FileHandle>,
    cover: Option<image::Handle>,
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
                rfd::AsyncFileDialog::new()
                    .add_filter("Music", &["wav", "flac", "ogg", "mp3"])
                    .pick_file(),
                MediaPlayerMessage::FileDialogHandle,
            ),
            MediaPlayerMessage::FileDialogHandle(handle) => {
                self.file_handle = handle.to_owned();

                Task::perform(
                    async {
                        if let Some(file_handle) = handle {
                            let tagged_file = lofty::read_from_path(file_handle.path()).unwrap();

                            tagged_file.primary_tag().map(|tag| {
                                let pic = tag.pictures().first().unwrap().data();
                                image::Handle::from_bytes(pic.to_owned())
                            })
                        } else {
                            None
                        }
                    },
                    MediaPlayerMessage::ImageHandle,
                )
            }
            MediaPlayerMessage::ImageHandle(image_handle) => {
                self.cover = image_handle;
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<MediaPlayerMessage> {
        Subscription::batch([self.subscription_tick()])
    }

    pub fn subscription_tick(&self) -> Subscription<MediaPlayerMessage> {
        if !self.controls.is_paused() && !self.controls.is_empty() {
            // NOTE: update view every 100 millisecond when music is playing.
            time::every(time::Duration::from_millis(100)).map(|_| MediaPlayerMessage::Tick)
        } else {
            Subscription::none()
        }
    }

    pub fn view(&self) -> Element<MediaPlayerMessage> {
        let playback_info = &self.controls.playback_info;
        let playing = !self.controls.is_paused() && !self.controls.is_empty();

        let mut play_button = button("Play").style(ui::button::success);
        let mut pause_button = button("Pause").style(ui::button::primary);
        let mut stop_button = button("Stop").style(ui::button::danger);

        if self.file_handle.is_some() || self.controls.is_paused() {
            play_button = play_button.on_press(MediaPlayerMessage::Play);
        }
        if !self.controls.is_empty() {
            pause_button = pause_button.on_press(MediaPlayerMessage::Pause);
            stop_button = stop_button.on_press(MediaPlayerMessage::Stop);
        }

        let stats = text(if self.controls.is_empty() {
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
        .size(12);

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

        let cover = container(
            self.cover
                .as_ref()
                .map_or(image("./fallback.png"), image)
                .height(iced::Length::Fill)
                .content_fit(iced::ContentFit::Contain),
        )
        .padding(20);

        let volume = slider(
            0.0..=1.0,
            self.controls.get_volume(),
            MediaPlayerMessage::Volume,
        )
        .step(0.1)
        .width(100);

        let content = container(
            row![
                button("Pick File")
                    .on_press(MediaPlayerMessage::FileDialogOpen)
                    .style(ui::button::primary),
                text(
                    self.file_handle
                        .as_ref()
                        .map(|handle| handle.path().to_str().unwrap())
                        .unwrap_or("No file selected")
                )
            ]
            .align_y(iced::Center)
            .spacing(5),
        )
        .padding(10);

        let controls = stack![
            column![
                vertical_space().height(10),
                container(
                    row![
                        if playing { pause_button } else { play_button },
                        stop_button,
                        volume,
                        horizontal_space(),
                        stats
                    ]
                    .align_y(iced::Center)
                    .spacing(10)
                    .padding(10)
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
        ];

        let gui = column![content, cover, controls].align_x(iced::Alignment::Center);

        gui.into()
    }
}
