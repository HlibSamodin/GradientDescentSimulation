use iced::Color;
use crate::widgets::widget_theme_picker::Theme;

// цвета панели
pub struct ColorScheme {
    pub panel_bg:  Color,
    pub center_bg: Color,
}

impl Theme {
    pub fn scheme(&self) -> ColorScheme {
        match self {
            Theme::Purple => ColorScheme {
                panel_bg:  Color::from_rgb(0.12, 0.10, 0.18),
                center_bg: Color::from_rgb(0.09, 0.08, 0.14),
            },
            Theme::Blue => ColorScheme {
                panel_bg:  Color::from_rgb(0.09, 0.11, 0.18),
                center_bg: Color::from_rgb(0.07, 0.09, 0.15),
            },
            Theme::Green => ColorScheme {
                panel_bg:  Color::from_rgb(0.09, 0.14, 0.11),
                center_bg: Color::from_rgb(0.07, 0.11, 0.09),
            },
            Theme::Yellow => ColorScheme {
                panel_bg:  Color::from_rgb(0.15, 0.13, 0.08),
                center_bg: Color::from_rgb(0.11, 0.10, 0.06),
            },
        }
    }
}
