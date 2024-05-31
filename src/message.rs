use crate::PFDeployment;

#[derive(Debug,Clone)]
pub enum Message {
    FilterDeployment(String),
    ListDeployment(Vec<PFDeployment>),
    SelectNamespace(String),
    Choose(String),
    Ignore,
    Load,
    Forward{name:String,forward:u16},
    SaveConfig,
    LoadConfig,
    InputForward{name:String,port:String},
    Error(String,u8)
}