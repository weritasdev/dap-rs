use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Jwk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

impl Jwk {
    pub fn compute_thumbprint(&self) -> Result<String, serde_json::Error> {
        let mut thumbprint_payload = serde_json::Map::new();
        if let Some(ref crv) = self.crv {
            thumbprint_payload.insert("crv".to_string(), serde_json::Value::String(crv.clone()));
        }
        if let Some(ref kty) = self.kty {
            thumbprint_payload.insert("kty".to_string(), serde_json::Value::String(kty.clone()));
        }
        if let Some(ref x) = self.x {
            thumbprint_payload.insert("x".to_string(), serde_json::Value::String(x.clone()));
        }
        if let Some(ref y) = self.y {
            thumbprint_payload.insert("y".to_string(), serde_json::Value::String(y.clone()));
        }

        let json_payload = serde_json::to_string(&thumbprint_payload)?;

        let mut hasher = Sha256::new();
        hasher.update(json_payload.as_bytes());

        let digest = hasher.finalize();
        let thumbprint = general_purpose::URL_SAFE_NO_PAD.encode(digest);

        Ok(thumbprint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hi() {
        let jwk = Jwk {
            alg: Some("ES256".to_string()),
            kty: Some("EC".to_string()),
            crv: Some("P-256".to_string()),
            x: Some("foo".to_string()),
            y: Some("bar".to_string()),
            d: None,
        };

        let thumbprint = jwk.compute_thumbprint().unwrap();
        println!("{}", thumbprint);
    }
}
