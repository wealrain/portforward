use std::fmt::format;

use iced::widget::{button, column, row, text};
use iced::{window, Command, Length, Settings, Size};
use iced::multi_window::{self,Application};
use crate::{theme, widget_namespace, widget_search_bar, Container, Element, Entry, EntryList, ForwardBox, Message, PFDeployment};
use crate::config::{Config, DataConfig, ForwardInfo};

const WINDOW_SIZE: Size = Size::new(780.0, 720.0);
fn application_icon() -> iced::window::Icon {
    let icon = include_bytes!("../assets/img/logo/icon.png");
    iced::window::icon::from_file_data(icon, None).unwrap()
}

#[derive(Default)]
pub struct App {
    deployments: EntryList,
    filter_deployments: EntryList,
    data_config: DataConfig,
    forward_box: ForwardBox,
}

impl App {
    pub fn launch() -> iced::Result {
        let config = Config::load();
        Self::run(Self::settings(config))
    }

    pub fn settings(config: Config) -> Settings<Config> {
        iced::Settings {
           flags: config, 
           window: window::Settings {
            icon: Some(application_icon()),
            size: WINDOW_SIZE,
            min_size: Some(WINDOW_SIZE),
            ..Default::default()
           },
           ..Default::default()
        }
    }

    pub fn select(&mut self,name: String) {
        for entry in self.filter_deployments.entries.iter_mut() {
            if entry.name == name {
                entry.selected = true;
            } else {
                entry.selected = false;
            }
        }

        for entry in self.deployments.entries.iter_mut() {
            if entry.name == name {
                entry.selected = true;
            } else {
                entry.selected = false;
            }
        }
    }
}

fn load_deployment(namespace: String) -> Command<Message> {
    let namespace = namespace.clone();
    Command::perform(PFDeployment::list_deployment(namespace),|v|{
        
        match v {
            Ok(list) => Message::ListDeployment(list),
            Err(e) => Message::Error(format!("{}",e),0),
        }
    })
}

fn port_forward(name: String,forward: u16) -> Command<Message> {
    let name = name.clone();
    Command::perform(PFDeployment::port_forward(name, forward),|v|{
        match v {
            Ok(_) => Message::Ignore,
            Err(e) => Message::Error(format!("{}",e),1),
        }
    })
}



impl multi_window::Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = Config;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = Self::default(); 
        (app, Command::none())
    }

    fn title(&self, _id: iced::window::Id) -> String {
        "Port Forward".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ListDeployment(v) => {
                self.data_config.list_deployment_error.clear();
                self.deployments.entries.clear();
                self.filter_deployments.entries.clear();
                for deployment in v {
                    self.deployments.entries.push(Entry { 
                        name: deployment.name.clone(), 
                        selected: false,
                        successed: false 
                    });

                    self.filter_deployments.entries.push(Entry { 
                        name: deployment.name.clone(), 
                        selected: false,
                        successed: false 
                    });

                    self.data_config.forward_infos.insert(deployment.name.clone(), ForwardInfo{
                        name: deployment.name.clone(),
                        port: "".into(),
                        forward: "".into(),
                    });
                }
                self.data_config.entries = self.deployments.entries.len();
                self.data_config.successed = 0;
                Command::none()
            },
            Message::FilterDeployment(v) => {
                // 原始数据得保留，过滤显示数据
                self.data_config.search_value = v.clone();
                let temp: Vec<&Entry> = self.deployments.entries.iter().filter(|entry| {
                    entry.name.contains(&v)
                }).collect();
                self.filter_deployments.entries.clear();
                for deployment in temp {
                    self.filter_deployments.entries.push(Entry { 
                        name: deployment.name.clone(), 
                        selected: deployment.selected,
                        successed: deployment.successed 
                    });  
                }

                Command::none()
            }
            Message::Load => {
                self.data_config.search_value = "".into();
                self.forward_box = ForwardBox::None;
                load_deployment(self.data_config.namespace.clone()) 
            }
            Message::Choose(name) => {
                self.select(name.clone());
                let forward_info = self.data_config.forward_infos.get(name.clone().as_str()).unwrap();
                self.data_config.forward_infos.insert(name.clone(), ForwardInfo{
                    name: forward_info.name.clone(),
                    port: forward_info.port.clone(),
                    forward: forward_info.forward.clone(),
                });
                
                self.forward_box = ForwardBox::Selected { name };
                Command::none()
            }
            Message::Forward{name,forward} => {
                for entry in self.filter_deployments.entries.iter_mut() {
                    if entry.name == name {
                        entry.successed = true;
                    }
                }

                for entry in self.deployments.entries.iter_mut() {
                    if entry.name == name {
                        entry.successed = true;
                    }
                }

                self.data_config.successed = self.deployments.entries.iter().filter(|entry| {
                    entry.successed
                }).count();
               port_forward(name.clone(), forward)
            }
            Message::InputForward{name,port} => {
                let forward_info = self.data_config.forward_infos.get(name.as_str()).unwrap();
                self.data_config.forward_infos.insert(name, ForwardInfo{
                    name: forward_info.name.clone(),
                    port: forward_info.port.clone(),
                    forward: port.clone(),
                });
                Command::none()
            }
            Message::Error(v,t) => {
                if t == 0 {
                    self.data_config.list_deployment_error = v;
                } else if t == 1{
                    self.forward_box = ForwardBox::Error(v);
                }
                
                Command::none()
            }
            Message::SelectNamespace(v) => {
                self.data_config.namespace = v.clone();
                Command::none()
            }
            _ => {
                Command::none()
            }
        }
    }

    fn view(
        &self,
        _id: iced::window::Id,
    ) -> Element<Message> {
        // left pane
        let namespace_box = widget_namespace(&self.data_config);
        let forward_box = self.forward_box.view(&self.data_config);

        let left_view = column![
            namespace_box,
            forward_box,
        ].spacing(10);

        // right pane
        let search_bar = widget_search_bar(&self.data_config);
        let info_bar = row![
            text(format!("DEPLOYMENTS: {}    FORWARDING: {}",self.data_config.entries,self.data_config.successed)),
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center);

        let entry_list = self.filter_deployments.view(self.data_config.list_deployment_error.clone());
       
        let right_view = column![
             search_bar,
             info_bar,
             entry_list,
        ]
        .spacing(10)
        .width(Length::FillPortion(2));

        let center = row![
            left_view,
            right_view,
        ]
        .spacing(10)
        .width(Length::Fill);

        // todo
        // let bottom_buttons = row![
        //     button("Save Config").on_press(Message::SaveConfig).style(theme::Button::Primary),
        //     button("Load Config").on_press(Message::LoadConfig).style(theme::Button::Primary),
        // ]
        // .spacing(8)
        // .align_items(iced::Alignment::Center);

        let main = column![
            center,
            // bottom_buttons
        ].spacing(10);
       
        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .into()
    }

}