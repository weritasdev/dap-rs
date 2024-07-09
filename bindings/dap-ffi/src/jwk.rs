use uniffi::{Object, Record};

#[derive(Debug, Clone, PartialEq, Eq, Object)]
#[uniffi::export(Debug, Eq)]
pub struct Jwk {
    inner: dap::jwk::Jwk,
}

#[uniffi::export]
impl Jwk {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: dap::jwk::Jwk {
                alg: None,
                kty: None,
                crv: None,
                d: None,
                x: None,
                y: None,
            },
        }
    }
}

/// Jwk record
#[derive(Record)]
pub struct JwkRecord {
    pub alg: Option<String>,
    pub kty: Option<String>,
    pub crv: Option<String>,
    pub d: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
}

impl From<dap::jwk::Jwk> for JwkRecord {
    fn from(jwk: dap::jwk::Jwk) -> Self {
        JwkRecord {
            alg: jwk.alg,
            kty: jwk.kty,
            crv: jwk.crv,
            d: jwk.d,
            x: jwk.x,
            y: jwk.y,
        }
    }
}

impl Into<dap::jwk::Jwk> for JwkRecord {
    fn into(self) -> dap::jwk::Jwk {
        dap::jwk::Jwk {
            alg: self.alg,
            kty: self.kty,
            crv: self.crv,
            d: self.d,
            x: self.x,
            y: self.y,
        }
    }
}
