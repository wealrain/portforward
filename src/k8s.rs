use std::fmt::Debug;
use std::net::SocketAddr;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube::api::ListParams;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpListener,
};
use tokio_stream::wrappers::TcpListenerStream;
use crate::{PFError, Result};


#[derive(Clone)]
pub struct PFPod {
    pub name: String,
    pub namespace: String,
    pub port: u16,
    pub forward: u16,
    client: kube::Client,
}

impl Debug for PFPod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, port: {}, forward: {}", self.name,self.port,self.forward)
    }
}

impl PFPod {
    
    pub async fn port_forward(&self,forward: u16) -> Result<()> {
        let api = kube::Api::<Pod>::namespaced(self.client.clone(), &self.namespace);
        let addr = SocketAddr::from(([127, 0, 0, 1], forward));
        let server = TcpListenerStream::new(TcpListener::bind(addr).await?)
            .take_until(tokio::signal::ctrl_c())
            .try_for_each(|conn| async {
                if let Ok(peer_addr) = conn.peer_addr() {
                    println!("{:?}", peer_addr);
                }
                let api = api.clone();
                let pod_name = self.name.clone();
                let port = self.port;
                tokio::spawn(async move{
                    if let Err(e) = Self::handle_connection(&api,pod_name.as_str(),port,conn).await {
                        eprintln!("{:?}", e);
                    }
                });
                Ok(())
            });
        
        if let Err(e) =server.await {
            eprintln!("{:?}", e);
        }

        Ok(())
    }

    async fn handle_connection(
        api: &kube::Api<Pod>, 
        pod_name: &str,
        port: u16,
        mut conn: impl AsyncRead + AsyncWrite + Unpin
    ) -> Result<()>{
        let mut forwarder = api.portforward(pod_name,&[port]).await?;
        let mut upstream_conn = forwarder.take_stream(port);
        tokio::io::copy_bidirectional(&mut conn, &mut upstream_conn.unwrap()).await?;
        // drop(upstream_conn);
        forwarder.join().await?;
        println!("port forwarding done");
        Ok(())
    } 
}

#[derive(Clone)]
pub struct PFDeployment {
    pub name: String,
    pub namespace: String,
    selector: LabelSelector,
    client: kube::Client
}

impl Debug for PFDeployment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"name is {} and namespace is {}",self.name,self.namespace)
    }
}

impl PFDeployment {
    pub async fn list_deployment(namespace: String) -> Result<Vec<PFDeployment>> {
        let client = kube::Client::try_default().await?;
        let api: kube::Api<Deployment> = kube::Api::namespaced(client.clone(), namespace.as_str());
        let list = api.list(&ListParams::default()).await?;
        let mut deployments = Vec::new();
        for deployment in list.items {
            let name = deployment.metadata.name.unwrap();
            let namespace = deployment.metadata.namespace.unwrap();
            let selector = deployment.spec.unwrap().selector;
            deployments.push(PFDeployment { 
                name,
                namespace,
                selector,
                client: client.clone()
            });
        }

        Ok(deployments)
    } 

    pub async fn find_deployment(name_space: &str,name: String) -> Result<Option<PFDeployment>> {
        let client = kube::Client::try_default().await?;
        let api: kube::Api<Deployment> = kube::Api::namespaced(client.clone(), name_space);
        let deployment = api.get(name.as_str()).await;
        if let std::result::Result::Ok(d) = deployment {
            let name = d.metadata.name.unwrap();
            let namespace = d.metadata.namespace.unwrap();
            let selector = d.spec.unwrap().selector;
            return Ok(Some(PFDeployment { 
                name,
                namespace,
                selector,
                client: client.clone()
            }));
        }

        Ok(None)
    } 

    pub async fn find_pod(&self) -> Result<Option<PFPod>> {
        let api: kube::Api<Pod> = kube::Api::namespaced(self.client.clone(), self.namespace.as_str());
        let mut label_selector = "".into();
        if let Some(selector) = self.selector.clone().match_labels {
            label_selector = selector.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(",");
        }

        let list_options = ListParams::default().labels(label_selector.as_str());
        let list = api.list(&list_options).await?;
        if list.items.len() == 0 {
            return Ok(None);
        }

        for pod in list.items {
            let name = pod.metadata.name.unwrap();
            let namespace = pod.metadata.namespace.unwrap();
            let port = if let Some(spec) = pod.spec {
                if let Some(ports) = spec.containers[0].ports.as_ref() {
                    ports[0].container_port
                } else { 0 }
            } else { 0 };
            return Ok(Some(PFPod {
                name,
                namespace,
                port: port as u16,
                forward: port as u16,
                client: self.client.clone(),
            }));
        }
        

        Ok(None)
    } 

    pub async fn port_forward(namespace:String,name:String,port:u16) -> Result<()> {
        let deployment = Self::find_deployment(namespace.as_str(), name.clone()).await?; 
        if let Some(deployment) = deployment {
            let pod = deployment.find_pod().await?;
            if let Some(pod) = pod {
                pod.port_forward(port).await?;
                return Ok(());
            } 
            return Err(Box::new(PFError::ResourceNotFound("Pod".into())));
             
        }
        return Err(Box::new(PFError::ResourceNotFound("Deployment".into())));
    }
}

 