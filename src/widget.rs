use iced::{
    alignment::Horizontal, 
    widget::{ button, column, container, row, scrollable, text, text_input, Column, Space}, 
    Length
};
use once_cell::sync::Lazy;
use crate::{config::{DataConfig, DeploymentConfig}, theme, Container, Element, Message, Text};
// tools
fn centerd_container<'a,Message>(
    content: impl Into<Element<'a,Message>>
) -> Container<'a,Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()     
}

fn centered_text<'a>(input: impl ToString) -> Text<'a> {
    text(input).horizontal_alignment(Horizontal::Center)
}

fn text_adv<'a>(str: impl ToString) -> Text<'a> {
    text(str).shaping(text::Shaping::Advanced)
}

 // namespace view
 pub fn widget_namespace(data_config:&DataConfig) -> Element<Message> {
    let title = "Load Data";
    let namespace = data_config.current_namespace.as_str();
    let input = text_input("input namespace",namespace)
        .on_input(|v| Message::SelectNamespace(v.clone()))
        .style(theme::TextInputStyle::Inverted);
    let button = button("Load Data")
        .on_press(Message::Load)
        .style(theme::Button::Primary);

        let content = column![
            input,
            button,
        ].spacing(8);

        let content = centerd_container(content
            .align_items(iced::Alignment::Start)
            .spacing(5)
            .padding(8)
        );

    container(
        Column::new().spacing(10)
        .push(title)
        .push(
            container(content)
            .padding(8)
            .style(theme::Container::Frame)
            .width(Length::Fill)
            .height(Length::Fill)
        )
    )
    .width(Length::Fill)
    .height(150.0)
    .into()
 }


// search bar
pub static SEARCH_BAR_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
pub fn widget_search_bar(data_config:&DataConfig) -> Element<Message> {
    let search_value = data_config.search_value.as_str();
    let input = text_input("input deployment name",search_value)
        .id(SEARCH_BAR_ID.clone())
        .on_input(|v| Message::FilterDeployment(v.clone()));
    
    let button = button("Search")
        .on_press(Message::Ignore)
        .style(theme::Button::Search);

    row![input, button]
        .spacing(5)
        .width(Length::FillPortion(1))
        .into()
}


// entry list

fn widget_view_entry((_index,entry):(usize,&Entry)) ->Element<Message> {
    // let check = checkbox("", entry.selected)
    //     .on_toggle(move |selected| Message::SelectDeployment {name:entry.name.clone(),selected})
    //     .style(theme::CheckBox::Entry);
    let name_text = text_adv(entry.name.clone());

    let view = row![name_text]
        .spacing(4)
        .padding(1)
        .align_items(iced::Alignment::Center);
    button(view)
        .width(Length::Fill)
        .on_press(Message::Choose(entry.name.clone()))
        .padding(4)
        .style(if entry.succeed {theme::Button::Start} else if entry.selected {theme::Button::Primary} else {theme::Button::Entry})
        .into()
}

#[derive(Debug,Clone)]
pub struct Entry {
    pub name: String,
    pub selected: bool,
    pub succeed: bool,
}

#[derive(Debug,Default)]
pub struct EntryList {
    pub entries: Vec<Entry>
}

impl EntryList {
    pub fn view(&self,error: String) ->Element<Message> {
        let entries = &self.entries;
        if !error.is_empty() {
            return centerd_container(
                column![]
                .push(centered_text(error.clone()))
                .align_items(iced::Alignment::Center)
            ).style(theme::Container::BlackHovered(false)).into();
        }
        if entries.is_empty() {
            return centerd_container(
                column![]
                .push(centered_text("Not Found"))
                .align_items(iced::Alignment::Center)
            ).style(theme::Container::BlackHovered(false)).into();
        }

        centerd_container(scrollable(row![
            column(entries.iter().enumerate().map(widget_view_entry))
            .spacing(10)
            .padding(5),
            Space::with_width(15)
        ]))
        .style(theme::Container::BlackHovered(false))
        .padding(5)
        .into()
    }
}

// forward pane
#[derive(Debug,Default,Clone)]
pub enum ForwardBox {
    #[default]
    None,
    Selected,
    Error(String)
}

impl ForwardBox {
    pub fn view(&self,data_config:&DataConfig) -> Element<Message> {
        let title = "Forward";

        let content = match &self {
            ForwardBox::Error(v) => column![text(v.clone())],
            ForwardBox::None => column![text("None Selected")],
            ForwardBox::Selected => {
                let port = data_config.current_port.clone();
                let name = data_config.current_deployment.clone();
                let forward_input = text_input("forward port",port.clone().as_str())
                .on_input(|v| Message::InputForward{port:v.clone()})
                .style(theme::TextInputStyle::Inverted);

                let button = button("forward").on_press(Message::Forward{
                    name:name.clone(),
                    port:port.parse::<u16>().unwrap_or(0),
                });

                column![
                    forward_input,
                    button,
                ].spacing(10)

            }
        };

        let content = centerd_container(content
            .align_items(iced::Alignment::Start)
            .spacing(5)
            .padding(8)
        );

        container(
            Column::new().spacing(10)
            .push(title)
            .push(
                container(content)
                .padding(8)
                .style(theme::Container::Frame)
                .width(Length::Fill)
                .height(Length::Fill)
            )
        )
        .width(Length::Fill)
        .height(150.0)
        .into()

    }
}
