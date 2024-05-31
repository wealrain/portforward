mod k8s;
mod app;
mod theme;
mod config;
mod widget;
mod message;
mod error;

pub use k8s::PFDeployment;
pub use app::App;
pub use widget::*;
pub use message::Message;
pub use error::PFError;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type Renderer = iced::Renderer;
pub type Theme = theme::Theme;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;
pub type Content<'a, Message> = iced::widget::pane_grid::Content<'a, Message, Theme, Renderer>;
pub type TitleBar<'a, Message> = iced::widget::pane_grid::TitleBar<'a, Message, Theme, Renderer>;
pub type Column<'a, Message> = iced::widget::Column<'a, Message, Theme, Renderer>;
pub type Row<'a, Message> = iced::widget::Row<'a, Message, Theme, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Theme, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Theme, Renderer>;
pub type Button<'a, Message> = iced::widget::Button<'a, Message, Theme, Renderer>;
pub type PickList<'a, Message, T, L, V> = iced::widget::PickList<'a, T, L, V, Message, Theme, Renderer>;