use std::path::PathBuf;

use crate::PFDeployment;

#[derive(Debug,Clone)]
pub enum Message {
    FilterDeployment(String),
    ListDeployment(Vec<PFDeployment>),
    SelectNamespace(String),
    Choose(String),
    Ignore,
    Load,
    Forward{name:String,port:u16},
    SaveConfig(Option<PathBuf>),
    LoadConfig(Option<PathBuf>),
    InputForward{port:String},
    Forwarded(bool),
    Error(String,u8),
    SaveConfigDialog,
    LoadConfigDialog
}