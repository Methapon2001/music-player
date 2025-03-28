use iced::{widget::button, Theme};

#[allow(dead_code)] // NOTE: This will be use in the future
pub fn primary(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        border: iced::Border::default().rounded(7.0),
        ..button::primary(theme, status)
    }
}

pub fn success(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        border: iced::Border::default().rounded(7.0),
        ..button::success(theme, status)
    }
}

pub fn danger(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        border: iced::Border::default().rounded(7.0),
        ..button::danger(theme, status)
    }
}
