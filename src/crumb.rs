use std::collections::HashMap;
use std::convert::TryFrom as _;
use std::ops::Deref;

use wreq::{Client, StatusCode, Uri};

use tokio::sync::{self, OnceCell, RwLock};
use tokio::time::{Duration, Instant};

use crate::YahooError;

static CSRF_TOKEN_REGEX: OnceCell<regex::Regex> = sync::OnceCell::const_new();

static SESSION_ID_REGEX: OnceCell<regex::Regex> = sync::OnceCell::const_new();

pub(crate) struct Crumb {
    client: Client,
    data: RwLock<Option<CrumbData>>,
}

struct Consent {
    session_id: String,
    csrf_token: String,
}

struct CrumbData {
    value: String,
    expires_at: Instant,
}

impl Crumb {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            data: RwLock::new(None),
        }
    }

    pub async fn enrich(&self, url: &mut Uri) -> Result<(), YahooError> {
        let crumb = {
            let maybe_loaded = {
                let guard = self.data.read().await;

                match guard.as_ref() {
                    Some(data) if data.expires_at < Instant::now() => Some(data.value.clone()),
                    _ => None,
                }
            };

            if let Some(crumb) = maybe_loaded {
                crumb
            } else {
                let mut guard = self.data.write().await;

                if guard.is_none() {
                    let crumb = self.obtain_crumb().await?;

                    // we possibly should read this from the cookie during the obtain_crumb, but let's
                    // be lazy for now and just set it to what we know yahoo uses.
                    let expires_at = Instant::now() + Duration::from_secs(86400);

                    *guard = Some(CrumbData {
                        value: crumb,
                        expires_at,
                    });
                }

                guard.deref().as_ref().unwrap().value.clone()
            }
        };

        let query_param = format!("crumb={}", crumb);
        let query = match url.query() {
            Some(orig) => {
                let mut str = orig.to_string();
                if !str.is_empty() {
                    str += "&";
                }
                str += &query_param;
                str
            }
            None => query_param,
        };

        let mut new_uri = Uri::builder();
        if let Some(scheme) = url.scheme() {
            new_uri = new_uri.scheme(scheme.clone());
        }
        if let Some(authority) = url.authority() {
            new_uri = new_uri.authority(authority.clone());
        }
        new_uri = new_uri.path_and_query(&format!("{}?{}", url.path(), query));

        let new_uri = new_uri
            .build()
            .map_err(|e| YahooError::FetchFailed(e.to_string()))?;

        *url = new_uri;

        Ok(())
    }

    async fn obtain_crumb(&self) -> Result<String, YahooError> {
        self.obtain_crumb_attempt().await
        // for i in 0..10 {
        //     log::trace!("obtain crumb attempt {}", i);
        //     match self.obtain_crumb_attempt().await {
        //         Ok(crumb) => {
        //             log::trace!("got crumb after {} attempts: {}", i, crumb);
        //             return Ok(crumb);
        //         }
        //         Err(e) => {
        //             log::trace!("failed to obtain crumb in attempt {}: {}", i, e);
        //
        //             // tokio::sleep has a different return value from std::thread::sleep
        //             #[allow(clippy::let_unit_value)]
        //             let _ = sleep(Duration::from_secs(1)).await;
        //         }
        //     };
        // }
        //
        // Err(YahooError::FetchFailed(
        //     "failed to obtain the crumb in 10 attempts".into(),
        // ))
    }

    async fn obtain_crumb_attempt(&self) -> Result<String, YahooError> {
        // try the easy way first
        let _ = self
            .client
            .get("https://fc.yahoo.com")
            .send()
            .await
            .map_err(|e| YahooError::from_wreq_while(e, "obtaining the session cookie"))?;

        let resp = self
            .client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .send()
            .await
            .map_err(|e| YahooError::from_wreq_while(e, "obtaining the crumb"))?;

        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| YahooError::from_wreq_while(e, "reading the crumb from the response"))?;

        if status == StatusCode::OK {
            Ok(body)
        } else {
            log::trace!(
                "got status {} when trying get the crumb the easy way with response body:\n{}",
                status,
                body
            );

            drop(body);

            // the hard way
            let data = self.get_collect_consent_payload(self.get_consent().await?);

            let _ = self
                .client
                .post("https://consent.yahoo.com/v2/collectConsent")
                .form(&data)
                .send()
                .await
                .map_err(|e| YahooError::from_wreq_while(e, "sending the consent"))?;

            self.client
                .get("https://guce.yahoo.com/copyConsent")
                .query(&data)
                .send()
                .await
                .map_err(|e| YahooError::from_wreq_while(e, "copying the consent into cookies"))?;

            let resp = self
                .client
                .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
                .send()
                .await
                .map_err(|e| {
                    YahooError::from_wreq_while(e, "sending the request to get the crumb")
                })?;

            if resp.status() != StatusCode::OK {
                return Err(YahooError::FetchFailed(format!(
                    "expected successful request when getting crumb but got one with status {}",
                    resp.status()
                )));
            }

            resp.text()
                .await
                .map_err(|e| YahooError::from_wreq_while(e, "reading the crumb from the response"))
        }
    }

    fn get_collect_consent_payload(&self, consent: Consent) -> HashMap<&'static str, String> {
        HashMap::from([
            ("agree", "agree".to_string()),
            ("consentUUID", "default".to_string()),
            ("sessionId", consent.session_id),
            ("csrfToken", consent.csrf_token),
            ("originalDoneUrl", "https://finance.yahoo.com".to_string()),
            ("namespace", "yahoo".to_string()),
        ])
    }

    async fn get_consent(&self) -> Result<Consent, YahooError> {
        log::trace!("getting consent");

        let resp = self
            .client
            .get("https://guce.yahoo.com/consent")
            .send()
            .await
            .map_err(|e| YahooError::from_wreq_while(e, "starting consent negotiation"))?;

        // the response should give us a redirect to the actual consent page with the sessionId as
        // a query attribute.
        let body = if resp.status().is_redirection() {
            // get the url to redirect to and extract the sessionId from it.
            let full_url = match resp.headers().get(wreq::header::LOCATION) {
                None => {
                    return Err(YahooError::FetchFailed(
                        "Expected the location header".into(),
                    ))
                }
                Some(loc) => loc
                    .to_str()
                    .map_err(|e| {
                        YahooError::FetchFailed(format!(
                            "failed to get the url to redirect to: {}",
                            e
                        ))
                    })
                    .and_then(|loc| {
                        Uri::try_from(loc).map_err(|e| {
                            YahooError::FetchFailed(format!(
                                "failed to parse the redirect location: {}",
                                e
                            ))
                        })
                    })?,
            };

            log::trace!("we're being redirected to {} to get the consent", full_url);

            let resp = self.client.get(full_url).send().await.map_err(|e| {
                YahooError::UnexpectedResponse(
                    format!("failed to get the consent web page: {}", e),
                    e,
                )
            })?;

            resp.text()
        } else {
            resp.text()
        }
        .await
        .map_err(|e| YahooError::from_wreq_while(e, "reading the consent web page"))?;

        let csrf_regex = CSRF_TOKEN_REGEX
            .get_or_init(|| async {regex::Regex::new(r#"<input[^>]+?(name=("|')csrfToken("|')[^>]+?value=("|')(?<token1>[^"']+)("|'))|(value=("|')(?<token2>[^"']+)("|')[^>]+?name=("|')csrfToken("|'))[^>]*?>"#).unwrap()}).await;
        let session_id_regex = SESSION_ID_REGEX
            .get_or_init(|| async {regex::Regex::new(r#"<input[^>]+?(name=("|')sessionId("|')[^>]+?value=("|')(?<token1>[^"']+)("|'))|(value=("|')(?<token2>[^"']+)("|')[^>]+?name=("|')sessionId("|'))[^>]*?>"#).unwrap()}).await;

        let capture = csrf_regex.captures(&body).ok_or_else(|| {
            YahooError::FetchFailed(format!(
                "failed to find the csrf token in the consent web page:\n{}",
                &body.as_str()[0..1000]
            ))
        })?;

        let csrf_token = capture
            .name("token1")
            .or_else(|| capture.name("token2"))
            .ok_or_else(|| {
                YahooError::FetchFailed(format!(
                    "failed to extract the csrf token from the web page:\n{}",
                    &body.as_str()[0..1000]
                ))
            })?
            .as_str()
            .to_owned();

        let capture = session_id_regex.captures(&body).ok_or_else(|| {
            YahooError::FetchFailed(format!(
                "failed to find the session id in the consent web page:\n{}",
                &body.as_str()[0..1000]
            ))
        })?;

        let session_id = capture
            .name("token1")
            .or_else(|| capture.name("token2"))
            .ok_or_else(|| {
                YahooError::FetchFailed(format!(
                    "failed to extract the session_id from the web page\n:{}",
                    &body.as_str()[0..1000]
                ))
            })?
            .as_str()
            .to_owned();

        Ok(Consent {
            session_id,
            csrf_token,
        })
    }
}
