use crate::jwk::JwkRecord;
use uniffi::{Object, Record};

#[derive(Debug, Clone, PartialEq, Eq, Object)]
#[uniffi::export(Debug, Eq)]
pub struct DidDocument {
    inner: dap::did_document::DidDocument,
}

#[uniffi::export]
impl DidDocument {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: dap::did_document::DidDocument::new(),
        }
    }
}

/// VerificationMethod record
#[derive(Record)]
pub struct VerificationMethodRecord {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub public_key_jwk: JwkRecord,
}

impl From<dap::did_document::VerificationMethod> for VerificationMethodRecord {
    fn from(vm: dap::did_document::VerificationMethod) -> Self {
        VerificationMethodRecord {
            id: vm.id,
            r#type: vm.r#type,
            controller: vm.controller,
            public_key_jwk: vm.public_key_jwk.into(),
        }
    }
}

impl Into<dap::did_document::VerificationMethod> for VerificationMethodRecord {
    fn into(self) -> dap::did_document::VerificationMethod {
        dap::did_document::VerificationMethod {
            id: self.id,
            r#type: self.r#type,
            controller: self.controller,
            public_key_jwk: self.public_key_jwk.into(),
        }
    }
}

/// Service record
#[derive(Record)]
pub struct ServiceRecord {
    pub id: String,
    pub r#type: String,
    pub service_endpoint: Vec<String>,
}

impl From<dap::did_document::Service> for ServiceRecord {
    fn from(service: dap::did_document::Service) -> Self {
        ServiceRecord {
            id: service.id,
            r#type: service.r#type,
            service_endpoint: service.service_endpoint,
        }
    }
}

impl Into<dap::did_document::Service> for ServiceRecord {
    fn into(self) -> dap::did_document::Service {
        dap::did_document::Service {
            id: self.id,
            r#type: self.r#type,
            service_endpoint: self.service_endpoint,
        }
    }
}

/// DidDocument record
#[derive(Record)]
pub struct DidDocumentRecord {
    pub id: String,
    pub context: Option<Vec<String>>,
    pub controller: Option<Vec<String>>,
    pub also_known_as: Option<Vec<String>>,
    pub verification_method: Option<Vec<VerificationMethodRecord>>,
    pub authentication: Option<Vec<String>>,
    pub assertion_method: Option<Vec<String>>,
    pub key_agreement: Option<Vec<String>>,
    pub capability_invocation: Option<Vec<String>>,
    pub capability_delegation: Option<Vec<String>>,
    pub service: Option<Vec<ServiceRecord>>,
}

impl From<dap::did_document::DidDocument> for DidDocumentRecord {
    fn from(doc: dap::did_document::DidDocument) -> Self {
        DidDocumentRecord {
            id: doc.id,
            context: doc.context,
            controller: doc.controller,
            also_known_as: doc.also_known_as,
            verification_method: doc
                .verification_method
                .map(|vms| vms.into_iter().map(Into::into).collect()),
            authentication: doc.authentication,
            assertion_method: doc.assertion_method,
            key_agreement: doc.key_agreement,
            capability_invocation: doc.capability_invocation,
            capability_delegation: doc.capability_delegation,
            service: doc
                .service
                .map(|services| services.into_iter().map(Into::into).collect()),
        }
    }
}

impl Into<dap::did_document::DidDocument> for DidDocumentRecord {
    fn into(self) -> dap::did_document::DidDocument {
        dap::did_document::DidDocument {
            id: self.id,
            context: self.context,
            controller: self.controller,
            also_known_as: self.also_known_as,
            verification_method: self
                .verification_method
                .map(|vms| vms.into_iter().map(Into::into).collect()),
            authentication: self.authentication,
            assertion_method: self.assertion_method,
            key_agreement: self.key_agreement,
            capability_invocation: self.capability_invocation,
            capability_delegation: self.capability_delegation,
            service: self
                .service
                .map(|services| services.into_iter().map(Into::into).collect()),
        }
    }
}
