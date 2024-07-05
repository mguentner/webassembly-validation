use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::net::Ipv4Addr;

pub enum JsonRejection {
    JsonDataError { message: String, path: String },
    JsonSyntaxError { message: String },
}

pub fn from_bytes<T>(bytes: &[u8]) -> Result<T, JsonRejection>
where
    T: DeserializeOwned,
{
    let deserializer = &mut serde_json::Deserializer::from_slice(bytes);

    let value = match serde_path_to_error::deserialize(deserializer) {
        Ok(value) => value,
        Err(err) => {
            let rejection = match err.inner().classify() {
                serde_json::error::Category::Data => JsonRejection::JsonDataError {
                    message: err.to_string(),
                    path: err.path().to_string(),
                },
                serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
                    JsonRejection::JsonSyntaxError {
                        message: err.to_string(),
                    }
                }
                serde_json::error::Category::Io => JsonRejection::JsonDataError {
                    message: err.to_string(),
                    path: err.path().to_string(),
                },
            };
            return Err(rejection);
        }
    };

    Ok(value)
}

#[derive(Serialize, Deserialize)]
pub struct CreateHostParams {
    pub hostname: String,
    pub ipv4: Ipv4Addr,
}

#[derive(Serialize)]
pub struct Host {
    pub id: u64,
    pub hostname: String,
    pub ipv4: Ipv4Addr,
}

#[derive(Serialize, Deserialize)]
pub struct BusinessValidationError {
    pub message: String,
    pub path: String,
}

/// used for WebAssembly
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ValidationResult<T> {
    Success { validated: T },
    Error { error: String, path: Option<String> },
}

impl CreateHostParams {
    pub fn parse_str(s: &str) -> Result<CreateHostParams, JsonRejection> {
        from_bytes::<CreateHostParams>(s.as_bytes())
    }

    pub fn validate(&self) -> Result<(), BusinessValidationError> {
        let CreateHostParams { hostname, ipv4 } = self;
        let banned_hostnames = Vec::from(["localhost", "batman"]);
        if banned_hostnames.contains(&hostname.as_str()) {
            return Err(BusinessValidationError {
                message: "illegal hostname".to_owned(),
                path: "hostname".to_owned(),
            });
        }
        // non exhaustive, `is_global()` would be but is unstable and requires nightly
        if ipv4.is_loopback() || ipv4.is_broadcast() {
            return Err(BusinessValidationError {
                message: "illegal ipv4 address".to_owned(),
                path: "ipv4".to_owned(),
            });
        }
        Ok(())
    }

    /// to be called by WebAssembly targets
    pub fn parse_and_validate(s: &str) -> ValidationResult<CreateHostParams> {
        match CreateHostParams::parse_str(s) {
            Ok(validated) => {
                if let Err(BusinessValidationError { message, path }) = validated.validate() {
                    ValidationResult::Error {
                        error: message,
                        path: Some(path),
                    }
                } else {
                    ValidationResult::Success { validated }
                }
            }
            Err(JsonRejection::JsonDataError { message, path }) => ValidationResult::Error {
                error: message,
                path: Some(path),
            },
            Err(JsonRejection::JsonSyntaxError { message }) => ValidationResult::Error {
                error: message,
                path: None,
            },
        }
    }
}
