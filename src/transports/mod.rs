use crate::enums::EResult;


pub mod web_api;
pub mod web_socket_cm;
pub mod cm_server;
pub mod cm_list_cache;

pub use web_socket_cm::WebSocketCMTransport;

#[derive(Debug, Clone)]
pub struct ApiResponse {
    eresult: Option<EResult>,
    error_message: Option<String>,
    body: Option<Vec<u8>>,
}

pub struct ApiRequest {
    interface: String,
    method: String,
    version: u32,
    access_token: Option<String>,
    // todo proper data type, probably String or Vec<u8>,
    request_data: Option<u8>,
    // todo headers type
    headers: Option<u8>,
}

impl ApiRequest {
    pub fn pathname(&self) -> String {
        format!(
            "I{}Service/{}/v{}",
            self.interface,
            self.method,
            self.version
        )
    }
}

pub struct ApiRequest2 {
    interface: String,
    method: String,
    version: u32,
    access_token: Option<String>,
}

impl ApiRequest2 {
    pub fn pathname(&self) -> String {
        format!(
            "I{}Service/{}/v{}",
            self.interface,
            self.method,
            self.version
        )
    }
    
    pub fn target_name(&self) -> String {
        format!(
            "{}.{}#{}",
            self.interface,
            self.method,
            self.version,
        )
    }
}