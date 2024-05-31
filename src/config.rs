use std::collections::HashMap;

#[derive(Debug,Default,Clone)]
pub struct Config{

}

impl Config {
    pub fn load()-> Self {
        Self::default()
    }
}

#[derive(Debug,Clone,Default)]
pub struct DataConfig {
    pub search_value: String,
    pub namespace: String,
    pub entries: usize,
    pub successed: usize,
    pub forward_infos : HashMap::<String,ForwardInfo>,
    pub list_deployment_error: String
}

#[derive(Debug,Default,Clone)]
pub struct ForwardInfo {
    pub name: String,
    pub port: String,
    pub forward: String,
}

 