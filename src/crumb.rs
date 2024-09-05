#[cfg(feature = "blocking")]
use reqwest::blocking::Client;

#[cfg(not(feature = "blocking"))]
use reqwest::Client;

#[cfg(not(feature = "blocking"))]
use tokio::sync::RwLock;

use reqwest::Url;

use crate::YahooError;

pub(crate) struct Crumb {
    client: Client,

    #[cfg(not(feature = "blocking"))]
    value: RwLock<Option<String>>,

    #[cfg(feature = "blocking")]
    value: Option<String>,
}

impl Crumb {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            #[cfg(feature = "blocking")]
            value: None,
            #[cfg(not(feature = "blocking"))]
            value: RwLock::new(None),
        }
    }

    #[cfg(feature = "blocking")]
    pub fn enrich(&self, url: &mut Url) -> Result<(), YahooError> {
        let mut query = url.query_pairs_mut();
        match &self.value {
            Some(c) => {
                query.append_pair("crumb", c);
            }
            None => {
                let crumb = self.obtain_crumb()?;
                query.append_pair("crumb", &crumb);
            }
        };

        Ok(())
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn enrich(&self, url: &mut Url) -> Result<(), YahooError> {
        use std::ops::Deref;

        let crumb = {
            let lock = self.value.read().await;
            lock.deref().clone()
        };

        let crumb = match crumb {
            Some(c) => c,
            None => {
                let crumb = self.obtain_crumb().await?;
                let mut guard = self.value.write().await;
                *guard = Some(crumb.clone());
                crumb
            }
        };

        let mut query = url.query_pairs_mut();
        query.append_pair("crumb", &crumb);

        Ok(())
    }

    #[cfg(feature = "blocking")]
    fn obtain_crumb(&self) -> Result<String, YahooError> {
        // we only care about the cookie from this request and we assume that the client
        // is using a cookie store that will automagically store it.

        use reqwest::StatusCode;
        let _ = self
            .client
            .get("https://fc.yahoo.com")
            .send()
            .map_err(|e| {
                YahooError::FetchFailed(format!("failed to obtain the session cookie: {}", e))
            })?;

        let resp = self
            .client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .send()
            .map_err(|e| YahooError::FetchFailed(format!("failed to obtain the crumb: {}", e)))?;

        if resp.status() != StatusCode::OK {
            return Err(YahooError::FetchFailed(format!(
                "expected successful request when getting crumb but got one with status {}",
                resp.status()
            )));
        }

        resp.text().map_err(|e| {
            YahooError::FetchFailed(format!("failed to read the crumb from the response: {}", e))
        })
    }

    #[cfg(not(feature = "blocking"))]
    async fn obtain_crumb(&self) -> Result<String, YahooError> {
        // we only care about the cookie from this request

        use reqwest::StatusCode;
        let _ = self
            .client
            .get("https://fc.yahoo.com")
            .send()
            .await
            .map_err(|e| {
                YahooError::FetchFailed(format!("failed to obtain the session cookie: {}", e))
            })?;

        let resp = self
            .client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .send()
            .await
            .map_err(|e| YahooError::FetchFailed(format!("failed to obtain the crumb: {}", e)))?;

        if resp.status() != StatusCode::OK {
            return Err(YahooError::FetchFailed(format!(
                "expected successful request when getting crumb but got one with status {}",
                resp.status()
            )));
        }

        let ret = resp.text().await.map_err(|e| {
            YahooError::FetchFailed(format!("failed to read the crumb from the response: {}", e))
        })?;

        Ok(ret)
    }
}
