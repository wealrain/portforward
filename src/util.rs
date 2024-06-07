use std::path::PathBuf;

use iced::{window, Command};

use crate::{Message, PFDeployment};

pub fn load_deployment(id:window::Id, namespace: String) -> Command<Message> {
    let namespace = namespace.clone();
    Command::perform(PFDeployment::list_deployment(namespace),move|v|{
        
        match v {
            Ok(list) => Message::ListDeployment(id,list),
            Err(e) => Message::Error(id,format!("{}",e),0),
        }
    })
}

pub fn port_forward(id: window::Id,namespace:String,name: String,port: u16) -> Command<Message> {
    Command::perform(PFDeployment::port_forward(namespace,name, port),move|v|{
        match v {
            Ok(_) => Message::Ignore,
            Err(e) => Message::Error(id,format!("{}",e),1),
        }
    })
}

pub async fn file_dialog(id: window::Id) -> (Option<(window::Id,PathBuf)>) {
    rfd::AsyncFileDialog::new()
        .pick_file()
        .await
        .map(|f| (id,f.path().to_owned()))
}