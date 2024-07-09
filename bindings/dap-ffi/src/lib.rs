use uniffi::Object;

pub mod did_document;
pub mod jwk;

#[derive(Object)]
pub struct DapLibrary;

#[uniffi::export]
impl DapLibrary {
    #[inline]
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self
    }

    #[inline]
    pub fn git_hash_version(&self) -> Option<String> {
        option_env!("GIT_HASH").map(|v| v.to_string())
    }
}

uniffi::setup_scaffolding!();
