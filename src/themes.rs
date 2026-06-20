use iced::Color;
use crate::widgets::widget_theme_picker::Theme;

// цвета панели
#[derive(Clone, Copy)]
pub struct ColorScheme {
    pub panel_bg:  Color,
    pub center_bg: Color,
    pub slider_fill: Color,
    pub slider_track: Color,
}

impl Theme {
    pub fn scheme(&self) -> ColorScheme {
        match self {
            Theme::Purple => ColorScheme {
                panel_bg:  Color::from_rgb(0.12, 0.10, 0.18),
                center_bg: Color::from_rgb(0.09, 0.08, 0.14),
                slider_fill:  Color::from_rgb(0.65, 0.35, 0.95),
                slider_track: Color::from_rgb(0.20, 0.17, 0.28),
            },
            Theme::Blue => ColorScheme {
                panel_bg:  Color::from_rgb(0.09, 0.11, 0.18),
                center_bg: Color::from_rgb(0.07, 0.09, 0.15),
                slider_fill:  Color::from_rgb(0.20, 0.55, 0.95),
                slider_track: Color::from_rgb(0.15, 0.18, 0.28),
            },
            Theme::Green => ColorScheme {
                panel_bg:  Color::from_rgb(0.09, 0.14, 0.11),
                center_bg: Color::from_rgb(0.07, 0.11, 0.09),
                slider_fill:  Color::from_rgb(0.15, 0.80, 0.40),
                slider_track: Color::from_rgb(0.15, 0.22, 0.18),
            },
            Theme::Yellow => ColorScheme {
                panel_bg:  Color::from_rgb(0.15, 0.13, 0.08),
                center_bg: Color::from_rgb(0.11, 0.10, 0.06),
                slider_fill:  Color::from_rgb(0.95, 0.75, 0.10),
                slider_track: Color::from_rgb(0.25, 0.22, 0.15),
            },
        }
    }
}
