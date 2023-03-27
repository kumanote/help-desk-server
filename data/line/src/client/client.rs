use crate::{Error, Result};
use hyper::Uri;
use serde::{de::DeserializeOwned, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Client {
    inner: sealed::HttpClient,
}

impl Client {
    pub fn new(url: &str) -> Result<Self> {
        let url = Uri::from_str(url).map_err(|_| Error::UnsupportedUrl {
            url: url.to_owned(),
        })?;
        let scheme = url
            .scheme()
            .map(|scheme| scheme.as_str())
            .unwrap_or("https");
        let inner = match scheme {
            "https" => Ok(sealed::HttpClient::new_https(url)),
            _ => Err(Error::UnsupportedUrl {
                url: url.to_string(),
            }),
        }?;
        Ok(Self { inner })
    }

    pub async fn get<T>(&self, channel_access_token: &str) -> Result<T>
    where
        T: Sized + DeserializeOwned,
    {
        self.inner.get(channel_access_token).await
    }

    pub async fn post<P, T>(&self, params: P, channel_access_token: &str) -> Result<T>
    where
        P: Sized + Serialize,
        T: Sized + DeserializeOwned,
    {
        self.inner.post(params, channel_access_token).await
    }
}

mod sealed {
    use crate::{Error, ErrorResponse, Result};
    use hyper::{
        body::Buf,
        client::{connect::Connect, HttpConnector},
        header, StatusCode, Uri,
    };
    use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
    use serde::{de::DeserializeOwned, Serialize};
    use std::io::Read;

    #[derive(Debug, Clone)]
    pub struct HyperClient<C> {
        uri: Uri,
        inner: hyper::Client<C>,
    }

    impl<C> HyperClient<C> {
        pub fn new(uri: Uri, inner: hyper::Client<C>) -> Self {
            Self { uri, inner }
        }
    }

    #[derive(Debug, Clone)]
    pub enum HttpClient {
        /// line api supports only `HTTPS`.
        Https(HyperClient<HttpsConnector<HttpConnector>>),
    }

    impl<C> HyperClient<C>
    where
        C: Connect + Clone + Send + Sync + 'static,
    {
        pub async fn get<T>(&self, channel_access_token: &str) -> Result<T>
        where
            T: Sized + DeserializeOwned,
        {
            let mut request = hyper::Request::builder()
                .method("GET")
                .uri(&self.uri)
                .body(hyper::Body::empty())?;
            let headers = request.headers_mut();
            headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {}", channel_access_token).parse().unwrap(),
            );
            let response = self.inner.request(request).await?;
            let response_status = response.status().clone();
            let mut response_body = String::new();
            hyper::body::aggregate(response.into_body())
                .await?
                .reader()
                .read_to_string(&mut response_body)?;
            if response_status == StatusCode::OK {
                Ok(serde_json::from_str(&response_body)?)
            } else {
                if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_body) {
                    Err(Error::ErrorResponse(error_response))
                } else {
                    Err(Error::UnexpectedErrorResponse {
                        status_code: response_status.as_u16(),
                        response_body,
                    })
                }
            }
        }

        pub async fn post<P, T>(&self, params: P, channel_access_token: &str) -> Result<T>
        where
            P: Sized + Serialize,
            T: Sized + DeserializeOwned,
        {
            let payload = serde_json::to_string(&params)?;
            let mut request = hyper::Request::builder()
                .method("POST")
                .uri(&self.uri)
                .body(hyper::Body::from(payload.into_bytes()))?;
            let headers = request.headers_mut();
            headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {}", channel_access_token).parse().unwrap(),
            );
            let response = self.inner.request(request).await?;
            let response_status = response.status().clone();
            let mut response_body = String::new();
            hyper::body::aggregate(response.into_body())
                .await?
                .reader()
                .read_to_string(&mut response_body)?;
            if response_status == StatusCode::OK {
                Ok(serde_json::from_str(&response_body)?)
            } else {
                if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_body) {
                    Err(Error::ErrorResponse(error_response))
                } else {
                    Err(Error::UnexpectedErrorResponse {
                        status_code: response_status.as_u16(),
                        response_body,
                    })
                }
            }
        }
    }

    impl HttpClient {
        pub fn new_https(uri: Uri) -> Self {
            let https_connector = HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .enable_http2()
                .build();
            Self::Https(HyperClient::new(
                uri,
                hyper::Client::builder().build(https_connector),
            ))
        }

        pub async fn get<T>(&self, channel_access_token: &str) -> Result<T>
        where
            T: Sized + DeserializeOwned,
        {
            match self {
                HttpClient::Https(c) => c.get(channel_access_token).await,
            }
        }

        pub async fn post<P, T>(&self, params: P, channel_access_token: &str) -> Result<T>
        where
            P: Sized + Serialize,
            T: Sized + DeserializeOwned,
        {
            match self {
                HttpClient::Https(c) => c.post(params, channel_access_token).await,
            }
        }
    }
}
