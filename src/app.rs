use std::collections::HashMap;

use iced::widget::{button, checkbox, column, container, row, text, Space};
use iced::{window, Command, Length, Settings, Size};
use iced::multi_window::{self,Application};
use serde::de;
use crate::util::{file_dialog, load_deployment, port_forward};
use crate::{theme, widget_namespace, widget_search_bar, Container, Element, Entry, EntryList, ForwardBox, Message, PFDeployment, Row};
use crate::config::{Config, Deployment};

const WINDOW_SIZE: Size = Size::new(780.0, 720.0);
fn application_icon() -> iced::window::Icon {
    let icon = include_bytes!("../assets/img/logo/icon.png");
    iced::window::icon::from_file_data(icon, None).unwrap()
}

#[derive(Default)]
pub struct App {
    windows: HashMap<window::Id, Window>,
    next_window_pos: window::Position,
}

impl App {
    pub fn launch() -> iced::Result {
        Self::run(Self::settings())
    }

    pub fn settings() -> Settings<Config> {
        iced::Settings {
           window: window::Settings {
            icon: Some(application_icon()),
            size: WINDOW_SIZE,
            min_size: Some(WINDOW_SIZE),
            ..Default::default()
           },
           ..Default::default()
        }
    }

}

impl multi_window::Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = Config;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = Self{
            windows: HashMap::from([(window::Id::MAIN,Window::new())]),
            next_window_pos: window::Position::Default
        };
        (app, Command::none())
    }

    fn title(&self, _id: iced::window::Id) -> String {
        "Port Forward".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ListDeployment(id,v) => {
                let window = self.windows.get_mut(&id).expect("Window not found.");
                window.fill(v)
            }
            Message::FilterDeployment(id,v) => {
                let window = self.windows.get_mut(&id).expect("Window not found.");
                window.config.data_config.search_value = v.clone();
                window.filter();
            }
            Message::Load(id) => {
                let window = self.windows.get_mut(&id).expect("Window not found.");
                let namespace = window.config.data_config.current_namespace.clone();
                window.clear();
                window.config.data_config.current_namespace = namespace.clone();
                return load_deployment(id,namespace); 
            }
            Message::Choose(id,name) => {
                let window = self.windows.get_mut(&id).expect("Window not found.");
                window.select(name.clone());      
            }
            Message::Forward{id,name,port} => {
                let window = self.windows.get_mut(&id).expect("Window not found.");
                return window.forward(id,name, port);
            }
            Message::InputForward{id,port} => {
                let window = self.windows.get_mut(&id).expect("Window not found.");
                window.config.data_config.current_port = port;
            }
            Message::Error(id,v,t) => {
                let window = self.windows.get_mut(&id).expect("Window not found.");
                if t == 0 {
                    window.config.data_config.list_deployment_error = v;
                } else if t == 1{
                    window.forward_box = ForwardBox::Error(v);
                }
                
            }
            Message::SelectNamespace(id,v) => {
                let window = self.windows.get_mut(&id).expect("Window not found.");
                window.config.data_config.current_namespace = v.clone();
            }
            Message::Forwarded(id,v) => {
                let window = self.windows.get_mut(&id).expect("Window not found.");
                window.config.data_config.check_forwarded = v;
                window.filter();
            }
            Message::NewWindow => {
                let (id,spawn_window) = window::spawn(window::Settings { 
                    size: WINDOW_SIZE,
                    position: self.next_window_pos,
                    ..Default::default()
                });

                self.windows.insert(id, Window::new());

                return spawn_window;
            }
            Message::SaveConfigDialog(id) => {
                return Command::perform(file_dialog(id), Message::SaveConfig)
            }
            Message::LoadConfigDialog(id) => {
                return Command::perform(file_dialog(id), Message::LoadConfig)
            }
            Message::SaveConfig(path) => {
                if let Some((id,path)) = path {
                    let window = self.windows.get_mut(&id).expect("Window not found.");
                    let deployment_config = window.config.deployment_config.clone();
                    return Command::perform(async move { deployment_config.save(path).await }, |_| Message::Ignore);
                }
            }
            Message::LoadConfig(path) => {
                if let Some((id,path)) = path {
                    let window = self.windows.get_mut(&id).expect("Window not found.");
                    if let Ok(_) = window.config.deployment_config.load(path) {
                        let mut forward_command = Vec::<Command<Message>>::new();
                        window.clear();
                        
                        for (namespace,deployments) in window.config.deployment_config.deployments.iter() {
                            let mut count = 0;
                            window.config.data_config.current_namespace = namespace.clone();

                            for (name,deployment) in deployments.iter() {
                                let succeed = deployment.forwarded  == 1 ;
                               
                                window.filter_deployments.entries.push(Entry { 
                                    name: name.clone(), 
                                    selected: false,
                                    succeed
                                });
                                if succeed {
                                    count += 1;
                                    forward_command.push(port_forward(id,namespace.clone(),name.clone(), deployment.port));
                                }
                               
                            }
                            window.config.data_config.current_entries = deployments.len();
                            window.config.data_config.current_succeed = count;
                        }

                        return Command::batch(forward_command);

                    }
                }
            }

            _ => {
                
            }
        }
        Command::none()
    }

    fn view(
        &self,
        id: window::Id,
    ) -> Element<Message> {
        let content = self.windows.get(&id).unwrap().view(id);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        
    }

}

#[derive(Debug, Clone,Default)]
struct Window {
    filter_deployments: EntryList,
    forward_box: ForwardBox,
    config: Config,
}

 

impl Window {
    fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.config.data_config.clear();
        self.filter_deployments.entries.clear();
        self.forward_box = ForwardBox::None;
    }

    pub fn fill(&mut self,deployments: Vec<PFDeployment>) {
        let namespace = self.config.data_config.current_namespace.clone();
        self.clear();
        let deployment_map = self.config.deployment_config
            .deployments
            .entry(namespace.clone())
            .or_insert(HashMap::<String,Deployment>::new());
        
        let mut succeed_count = 0;

        for deployment in deployments {
            let v_deployment = deployment_map.entry(deployment.name.clone()).or_insert(Deployment::default());

            self.filter_deployments.entries.push(Entry { 
                name: deployment.name.clone(), 
                selected: false,
                succeed: v_deployment.forwarded == 1 
            });

            if v_deployment.forwarded == 1 {
                succeed_count += 1;
            }
        }
        self.config.data_config.current_namespace = namespace.clone();
        self.config.data_config.current_entries = deployment_map.len();
        self.config.data_config.current_succeed = succeed_count;
    }

    pub fn select(&mut self,name: String) {
        for entry in self.filter_deployments.entries.iter_mut() {
            if entry.name == name {
                entry.selected = true;
            } else {
                entry.selected = false;
            }
        }
        let namespace = self.config.data_config.current_namespace.clone();
        let deployments = self.config.deployment_config.deployments.get(namespace.as_str()).unwrap();
        let deployment = deployments.get(name.as_str()).unwrap();
        let port = deployment.port;
        self.config.data_config.current_deployment = name.clone();
        self.config.data_config.current_port = port.to_string();
        self.forward_box = ForwardBox::Selected;
    }

    pub fn filter(&mut self) {
        if self.config.deployment_config.deployments.len() == 0 {
            return;
        }

        let search_value = self.config.data_config.search_value.clone();
        let forwarded = self.config.data_config.check_forwarded;
        
        let deployments = self.config.deployment_config.deployments.get(self.config.data_config.current_namespace.clone().as_str()).unwrap();

        let temp:Vec<Entry> = deployments.iter().filter(|entry| {
            entry.0.contains(&search_value) && (if forwarded { entry.1.forwarded == 1} else { true })
        }).map(|v|{
            Entry { name: v.0.clone(), selected: false, succeed: v.1.forwarded == 1 }
        }).collect();
        self.filter_deployments.entries.clear();
        for deployment in temp {
            self.filter_deployments.entries.push(Entry { 
                name: deployment.name.clone(), 
                selected: deployment.selected,
                succeed: deployment.succeed 
            });  
        }
    }

    pub fn forward(&mut self,id:window::Id, name:String,port:u16) -> Command<Message> {
        let namespace = self.config.data_config.current_namespace.clone();
        for entry in self.filter_deployments.entries.iter_mut() {
            if entry.name == name {
                entry.succeed = true;
            }
        }

        let deployments = self.config.deployment_config.deployments.entry(namespace.clone()).or_insert(HashMap::<String,Deployment>::new());
        let deployment = deployments.entry(name.clone()).or_insert(Deployment::default());
        deployment.port = port;
        deployment.forwarded = 1;
        

        self.config.data_config.current_succeed = deployments.iter().filter(|entry| {
            entry.1.forwarded == 1
        }).count();
        return port_forward(id,namespace,name.clone(), port);
    }

    fn view(&self,id: window::Id) -> Element<Message> {
        
        let namespace_box = widget_namespace(id,&self.config.data_config);
        let forward_box = self.forward_box.view(id,&self.config.data_config);

        let left_view = column![
            namespace_box,
            forward_box,
        ].spacing(10);

        // right pane
        let search_bar = widget_search_bar(id,&self.config.data_config);
        let info_bar = row![
            text(format!("DEPLOYMENTS: {}    FORWARDING: {}",self.config.data_config.current_entries,self.config.data_config.current_succeed)),
            Space::with_width(Length::Fill),
            checkbox("FORWARDED",self.config.data_config.check_forwarded)
                .on_toggle(move |v| {Message::Forwarded(id,v)})
                .style(theme::CheckBox::Inverted)
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center);

        let entry_list = self.filter_deployments.view(id,self.config.data_config.list_deployment_error.clone());
       
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

        

        let bottom_buttons = row![
            button("Save Config").on_press(Message::SaveConfigDialog(id)).style(theme::Button::Primary),
            button("Load Config").on_press(Message::LoadConfigDialog(id)).style(theme::Button::Primary),
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center);

        let main = column![
            center,
            bottom_buttons
        ].spacing(10);
       
        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .into()
    }
}