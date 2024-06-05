use std::path::PathBuf;

use iced::Command;

use crate::{Message, PFDeployment};

pub fn load_deployment(namespace: String) -> Command<Message> {
    let namespace = namespace.clone();
    Command::perform(PFDeployment::list_deployment(namespace),|v|{
        
        match v {
            Ok(list) => Message::ListDeployment(list),
            Err(e) => Message::Error(format!("{}",e),0),
        }
    })
}

pub fn port_forward(namespace:String,name: String,port: u16) -> Command<Message> {
    Command::perform(PFDeployment::port_forward(namespace,name, port),|v|{
        match v {
            Ok(_) => Message::Ignore,
            Err(e) => Message::Error(format!("{}",e),1),
        }
    })
}

pub async fn file_dialog() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .pick_file()
        .await
        .map(|f| f.path().to_owned())
}