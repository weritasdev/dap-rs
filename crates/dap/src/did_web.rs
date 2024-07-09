use crate::did_document::DidDocument;
use reqwest::Error;

#[derive(Debug)]
pub enum DidResolutionError {
    MalformedDidDocument,
    ResolutionFailed,
}

pub async fn resolve(url: &str) -> Result<DidDocument, Error> {
    let response = reqwest::get(url).await?;
    let did_document = response.json::<DidDocument>().await?;

    // possible error scenarios:
    // - request could fail
    // - non 200 response
    // - failure to parse response body into DidDocument

    Ok(did_document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve() {
        let url = "https://didpay.me/.well-known/did.json";
        let did_document = resolve(url).await.unwrap();

        let to_print = serde_json::to_string_pretty(&did_document).unwrap();
        println!("{}", to_print);
    }
}
