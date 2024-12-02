use atoms_rpc_types::Block;
use serde::Serialize;

/// ResponseView decided if response of call will be returned as a string, json object or human readable format
#[derive(Debug, Clone)]
pub enum ResponseView {
    /// Response will be returned as a string
    String,
    /// Response will be returned as a json object
    Json,
    /// Response will be returned as a human readable format
    Human,
}

impl Default for ResponseView {
    fn default() -> Self {
        ResponseView::String
    }
}

impl ResponseView {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "string" => Some(ResponseView::String),
            "json" => Some(ResponseView::Json),
            "human" => Some(ResponseView::Human),
            _ => None,
        }
    }
}

#[derive(Serialize)]
pub enum Response {
    U64(u64),
    U128(u128),

    Bool(bool),
    String(String),

    Block(Block),
    Struct(serde_json::Value), // Use serde_json::Value for custom structs
}

impl Response {
    pub fn format(&self, view: ResponseView) -> String {
        match view {
            ResponseView::String => self.to_string(),
            ResponseView::Json => serde_json::to_string(self)
                .unwrap_or_else(|_| "Failed to serialize to JSON".to_string()),
            ResponseView::Human => self.to_human_readable(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Response::U64(val) => val.to_string(),
            Response::U128(val) => val.to_string(),
            Response::Bool(val) => val.to_string(),
            Response::String(val) => val.clone(),
            Response::Block(val) => serde_json::to_string(val)
                .unwrap_or_else(|_| "Failed to serialize to JSON".to_string()),
            Response::Struct(val) => val.to_string(),
        }
    }

    fn to_human_readable(&self) -> String {
        match self {
            Response::U64(val) => format!("u64 value: {:#?}", val),
            Response::U128(val) => format!("U128 value: {:#?}", val),
            Response::Bool(val) => format!("Boolean value: {:#?}", val),
            Response::String(val) => format!("String value: {:#?}", val),
            Response::Block(val) => format!("{:#?}", val),
            Response::Struct(val) => format!("Struct value: {:#?}", val),
        }
    }
}
