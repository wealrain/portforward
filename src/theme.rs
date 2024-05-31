use iced::application;
use iced::color;
use iced::widget::checkbox;
use iced::widget::scrollable;
use iced::widget::{button,text_input,container,text};
use iced::Background;
use iced::Border;
use iced::Color;

const BORDER_RADIUS: f32 = 5.0;
const BORDER_WIDTH: f32 = 1.5;
fn border(color: Color) -> Border {
    Border {
        color,
        width: BORDER_WIDTH,
        radius: BORDER_RADIUS.into(),
    }
}

#[derive(Default,Debug,PartialEq, Eq,Clone, Copy)]
pub enum Themems {
    #[default]
    Dark 
}

impl Themems {
    pub fn palette(&self) -> Palette {
        match self {
            Themems::Dark => Palette {
                middleground: color!(0x272727),
                foreground: color!(0x353535),
                background: color!(0x151515),
                text: color!(0xE0E0E0),
                accent: color!(0xFFCC00),
                success: color!(0x00FF00),
                border: color!(0x353535),
                error: color!(0xFF0000),
                warning: color!(0xFFCC00),
                peace: color!(0xBA84FC),
            },
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Palette {
    pub middleground: Color,
    pub foreground: Color,
    pub background: Color,
    pub text: Color,
    pub accent: Color,
    pub success: Color,
    pub border: Color,
    pub error: Color,
    pub warning: Color,
    pub peace: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Themems::Dark.palette()
    }
}

#[derive(Debug,Default,Clone,PartialEq)]
pub struct Theme(Palette);

impl Theme {
    pub fn palette(&self) -> &Palette {
        &self.0
    }
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette().middleground,
            text_color: self.palette().text
        }
    }
}

#[derive(Debug,Default,Clone, Copy,PartialEq, Eq)]
pub enum Button {
    #[default]
    Primary,
    Start,
    Entry,
    Search,
}

impl button::StyleSheet for Theme {
    type Style = Button;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        let palette = self.palette();
        let appearance = button::Appearance {
            ..button::Appearance::default()
        };

        let active_appearance = |bg: Option<Color>,mc| button::Appearance {
            background: Some(Background::Color(bg.unwrap_or(palette.foreground))),
            border: border(Color{a:0.5,..mc}),
            text_color: mc,
            ..appearance
        };
    
        match style {
            Button::Primary => active_appearance(None,palette.accent),
            Button::Start => active_appearance(None,palette.success),
            Button::Entry => button::Appearance {
                background: Some(Background::Color(palette.foreground)),
                text_color: palette.text,
                border: border(palette.border),
                ..appearance
            },
            Button::Search => active_appearance(None,palette.peace),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let palette = self.palette();
        
        let hover_appearance = |bg,tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.4, ..bg })),
            text_color: tc.unwrap_or(bg),
            ..active
        };
    
        match style {
            Button::Primary => hover_appearance(palette.accent,Some(palette.text)),
            Button::Start => hover_appearance(palette.success,Some(palette.text)),
            Button::Entry => button::Appearance {
                border: border(Color { a: 0.5, ..palette.accent }),
                ..hover_appearance(palette.accent, Some(palette.text))
            },
            Button::Search => hover_appearance(palette.peace,Some(palette.text)),
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }


}

#[derive(Default,Clone, Copy)]
pub enum TextInputStyle {
    #[default]
    Normal,
    Inverted
}

impl text_input::StyleSheet for Theme {
    type Style = TextInputStyle;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let p = self.palette();
        let default = text_input::Appearance {
            background: Background::Color(p.foreground),
            border: Border {
                color:p.border,
                width: BORDER_WIDTH,
                radius: BORDER_RADIUS.into()
            },
            icon_color: p.foreground
        };

        match style {
            TextInputStyle::Normal => default,
            TextInputStyle::Inverted =>text_input::Appearance {
                background: p.middleground.into(),
                ..default
            },
        }
    }
    
    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let p = self.palette();
        text_input::Appearance { 
            border: Border{
                color: p.accent,
                width: BORDER_WIDTH,
                radius: BORDER_RADIUS.into()
            }, 
            ..self.active(style)
        }
    }
    
    fn placeholder_color(&self, style: &Self::Style) -> Color {
        color!(0xFF, 0xFF, 0xFF,0.3)
    }
    
    fn value_color(&self, style: &Self::Style) -> Color {
       self.palette().accent
    }
    
    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.palette().text
    }
    
    fn selection_color(&self, style: &Self::Style) -> Color {
        Color {
            a: 0.5,
            ..self.palette().accent
        }
    }
    
    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}

#[derive(Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
    Error,
    Warning,
    Color(Color),
}
impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        let p = self.palette();

        match style {
            Text::Default => Default::default(),
            Text::Error => text::Appearance {
                color: Some(p.error),
            },
            Text::Warning => text::Appearance {
                color: Some(p.warning),
            },
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Container {
    #[default]
    Default,
    Hovered(bool),
    Frame,
    Black,
    BlackHovered(bool),
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let dark = container::Appearance {
            background: Some(self.palette().background.into()),
            text_color: Some(self.palette().text),
            border: border(self.palette().border),
            ..Default::default()
        };

        match style {
            Container::Default => container::Appearance::default(),
            Container::Frame => container::Appearance {
                background: Some(Background::Color(self.palette().foreground)),
                text_color: Some(self.palette().text),
                ..dark
            },
            Container::Black => dark,
            Container::BlackHovered(hovered) => match hovered {
                true => container::Appearance {
                    // border_color: ,
                    // border_width: 2.0,
                    border: Border {
                        width: 2.0,
                        color: Color {
                            a: 0.80,
                            ..self.palette().accent
                        },
                        radius: BORDER_RADIUS.into(),
                    },
                    ..dark
                },
                false => dark,
            },
            Container::Hovered(hovered) => match hovered {
                true => container::Appearance {
                    border: Border {
                        color: Color {
                            a: 0.80,
                            ..self.palette().accent
                        },
                        width: BORDER_WIDTH * 1.5,
                        radius: BORDER_RADIUS.into(),
                    },
                    ..container::Appearance::default()
                },
                false => container::Appearance::default(),
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    #[default]
    Normal,
    Dark,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> scrollable::Appearance {
        let p = self.palette();

        let border = Border {
            color: p.border,
            width: BORDER_WIDTH,
            radius: 3.0.into(),
        };

        let from_appearance = |c: Color, d: Color| scrollable::Appearance {
            gap: None,
            scrollbar: scrollable::Scrollbar {
                background: Some(Background::Color(c)),
                scroller: scrollable::Scroller { color: d, border },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    ..border
                },
            },
            container: container::Appearance::default(),
        };

        let color = (p.middleground, p.foreground);

        match style {
            Scrollable::Normal => from_appearance(color.0, color.1),
            Scrollable::Dark => from_appearance(color.1, color.0),
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> scrollable::Appearance {
        let p = self.palette();
        scrollable::Appearance {
            gap: None,
            scrollbar: scrollable::Scrollbar {
                scroller: scrollable::Scroller {
                    color: if is_mouse_over_scrollbar {
                        Color { a: 0.5, ..p.accent }
                    } else {
                        self.active(style).scrollbar.scroller.color
                    },
                    border: Border {
                        color: if is_mouse_over_scrollbar {
                            Color { a: 0.75, ..p.accent }
                        } else {
                            self.active(style).scrollbar.border.color
                        },
                        width: BORDER_WIDTH,
                        radius: 3.0.into(),
                    },
                },
                ..self.active(style).scrollbar
            },
            ..self.active(style)
        }
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Appearance {
        let hovered = self.hovered(style, true);

        scrollable::Appearance {
            scrollbar: scrollable::Scrollbar {
                ..hovered.scrollbar
            },
            ..hovered
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum CheckBox {
    #[default]
    Normal,
    Inverted,
    Entry,
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let p = self.palette();

        let default = checkbox::Appearance {
            background: p.middleground.into(),
            icon_color: p.accent,
            border: Border {
                color: p.border,
                width: BORDER_WIDTH,
                radius: BORDER_RADIUS.into(),
            },
            text_color: Some(p.text),
        };
        match style {
            CheckBox::Normal => default,
            CheckBox::Inverted => checkbox::Appearance {
                background: p.foreground.into(),
                ..default
            },
            CheckBox::Entry => checkbox::Appearance {
                // border_color: Color { a: 0.25, ..p.accent },
                ..default
            },
        }
    }

    // todo
    fn disabled(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        self.active(style, is_checked)
    }

    fn hovered(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let p = self.palette();

        let default = checkbox::Appearance {
            background: p.middleground.into(),
            icon_color: p.accent,
            border: Border {
                color: p.accent,
                width: 2.0,
                radius: BORDER_RADIUS.into(),
            },
            text_color: Some(p.text),
        };

        match style {
            CheckBox::Normal => default,
            CheckBox::Inverted => checkbox::Appearance {
                background: p.foreground.into(),
                ..default
            },
            CheckBox::Entry => default,
        }
    }
}


