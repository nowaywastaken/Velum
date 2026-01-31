
/// OOXML parsing errors
#[derive(Debug, thiserror::Error)]
pub enum OoxmlError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("ZIP archive error: {0}")]
    ZipError(#[from] zip::result::ZipError),
    
    #[error("Content-Type not found for part: {0}")]
    ContentTypeNotFound(String),
    
    #[error("Part not found: {0}")]
    PartNotFound(String),
    
    #[error("Relationship not found: {0}")]
    RelationshipNotFound(String),
    
    #[error("Invalid content type: {0}")]
    InvalidContentType(String),
    
    #[error("Missing required part: {0}")]
    MissingRequiredPart(String),
    
    #[error("Invalid namespace: {0}")]
    InvalidNamespace(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Unsupported content type: {0}")]
    UnsupportedContentType(String),
}
