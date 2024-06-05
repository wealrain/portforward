use std::{collections::HashMap, fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tracing::error;
use crate::{PFError, Result};

#[derive(Debug,Default,Clone)]
pub struct Config{
    pub deployment_config: DeploymentConfig,
    pub data_config: DataConfig
}


#[derive(Debug,Default,Clone,Serialize,Deserialize)]
pub struct DeploymentConfig {
    pub deployments: HashMap<String,HashMap<String,Deployment>>,
}

#[derive(Debug,Default,Clone,Serialize,Deserialize)]
pub struct Deployment {
    pub port: u16,
    pub forwarded: u8 // 0 false 1 true
}

impl DeploymentConfig {

    pub fn clear(&mut self) {
        self.deployments.clear();
    }

    pub fn load(&mut self,path: PathBuf) -> Result<()> {
        let Ok(cfg) = fs::read_to_string(path.as_path()) else {
            return Err(Box::new(PFError::LoadConfigBad));
        };
        let result = serde_json::from_str(&cfg).unwrap_or_else(|e|{
            error!("{}",e);
            Self::default()
        });

        self.deployments = result.deployments;

        Ok(())
    }

    pub async fn save(&self,path: PathBuf) -> Result<()>{
        let file = tokio::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path.as_path())
            .await;
        if let Err(e)  = &file{
            error!("{}",e);
            return Err(Box::new(PFError::WriteConfigBad));
        }

        let result = file?.write_all(serde_json::to_string_pretty(self)?.as_bytes()).await;
        if let Err(e)  = &result{
            error!("{}",e);
            return Err(Box::new(PFError::WriteConfigBad));
        }

        Ok(())
    }
}



#[derive(Debug,Clone,Default)]
pub struct DataConfig {
    pub destination: PathBuf,
    pub search_value: String,
    pub current_namespace: String,
    pub current_deployment: String,
    pub current_entries: usize,
    pub current_succeed: usize,
    pub current_port: String,
    pub list_deployment_error: String,
    pub check_forwarded: bool,
}

impl DataConfig {
    pub fn clear(&mut self) {
        self.search_value = "".to_string();
        self.current_port = "".to_string();
        self.current_namespace = "".to_string();
        self.current_deployment = "".to_string();
        self.current_entries = 0;
        self.current_succeed = 0;
        self.list_deployment_error = "".to_string();
        self.check_forwarded = false;
    }
}

 

 