use std::path::PathBuf;

use iced::window;

use crate::PFDeployment;

#[derive(Debug,Clone)]
pub enum Message {
    FilterDeployment(window::Id,String),
    ListDeployment(window::Id,Vec<PFDeployment>),
    SelectNamespace(window::Id,String),
    Choose(window::Id,String),
    NewWindow,
    Ignore,
    Load(window::Id),
    Forward{id:window::Id,name:String,port:u16},
    SaveConfig(Option<(window::Id,PathBuf)>),
    LoadConfig(Option<(window::Id,PathBuf)>),
    InputForward{id: window::Id,port:String},
    Forwarded(window::Id,bool),
    Error(window::Id,String,u8),
    SaveConfigDialog(window::Id),
    LoadConfigDialog(window::Id)
}