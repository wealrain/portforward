use thiserror::Error;

#[derive(Error, Debug)]
pub enum PFError {
    #[error("Resource {0} Not Found")]
    ResourceNotFound(String),
}