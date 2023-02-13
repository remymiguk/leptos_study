use crate::app::error::AppError;
use crate::app::error::ErrorInfo;
use log::info;
use reqwest::{header::LOCATION, Method, Url};
use serde::{de::DeserializeOwned, Serialize};

const API_ROOT: &str = "http://127.0.0.1:3000/";

// TODO: create
pub struct APIRequestReqwest {
    token: Option<String>,
    fetch_mode_no_cors: bool,
    navigate_url: Option<String>,
    body: Option<serde_json::Value>,
}

impl APIRequestReqwest {
    pub fn new() -> Self {
        Self {
            token: None,
            fetch_mode_no_cors: false,
            navigate_url: None,
            body: None,
        }
    }

    /// Build all kinds of http request: post/get/delete etc.
    pub async fn request<T>(&self, method: Method, url: &str) -> Result<T, AppError>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
    {
        info!("### url: {}...", url);

        let allow_body = matches!(method, Method::POST | Method::PUT | Method::PATCH);

        let url = format!("{API_ROOT}{url}");
        let url = Url::parse(&url).unwrap();

        // let redir_policy = match &self.navigate_url {
        //     Some(navigate_url) => {
        //         let navigate_url = navigate_url.clone();
        //         redirect::Policy::custom(move |attempt| {
        //             if attempt.previous().len() > 10 {
        //                 attempt.error("too many redirects")
        //             } else if attempt
        //                 .url()
        //                 .host_str()
        //                 .unwrap_or_default()
        //                 .starts_with(&navigate_url)
        //             {
        //                 // Prevent redirects to domain
        //                 attempt.stop()
        //             } else {
        //                 attempt.follow()
        //             }
        //         })
        //     }
        //     None => redirect::Policy::default(),
        // };

        let client = reqwest::Client::builder()
            // .redirect(redir_policy)
            .build()
            .unwrap();

        let mut builder = client
            .request(method, url)
            .header("Content-Type", "application/json");

        if self.fetch_mode_no_cors {
            builder = builder.fetch_mode_no_cors();
        }

        if let Some(token) = self.token.as_ref() {
            builder = builder.bearer_auth(token);
        }

        if allow_body {
            builder = builder.json(&self.body);
        }

        let response = builder.send().await;

        info!("### url response: {:?}", response);

        if let Ok(response) = response {
            if response.status().is_success() {
                let future = response.json::<T>();
                let data: Result<T, _> = future.await;
                match data {
                    Ok(data) => {
                        log::debug!("response: {:?}", data);
                        Ok(data)
                    }
                    Err(e) => Err(e.into()),
                }
            } else {
                let status = response.status().as_u16();
                match status {
                    302 | 303 | 307 => {
                        let location = response
                            .headers()
                            .get(LOCATION)
                            .map(|v| v.to_str().unwrap_or_default())
                            .unwrap_or_default()
                            .to_string();
                        Err(AppError::Redirect(location))
                    }
                    401 => Err(AppError::Unauthorized.into()),
                    403 => Err(AppError::Forbidden.into()),
                    404 => Err(AppError::NotFound.into()),
                    500 => Err(AppError::InternalServerError.into()),
                    422 => {
                        let future = response.json::<ErrorInfo>();
                        let data: Result<ErrorInfo, _> = future.await;
                        match data {
                            Ok(data) => Err(AppError::UnprocessableEntity(data).into()),
                            Err(e) => Err(e.into()),
                        }
                    }
                    e => Err(AppError::RequestError(format!("error `{e}`"))),
                }
            }
        } else {
            Err(AppError::RequestError(String::from("request error")))
        }
    }

    pub fn navigate_url(mut self, navigate_url: &str) -> Self {
        self.navigate_url = Some(navigate_url.to_string());
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn token_opt(mut self, token: Option<String>) -> Self {
        self.token = token;
        self
    }

    pub fn body<T: Serialize + std::fmt::Debug>(mut self, body: T) -> Result<Self, AppError> {
        self.body = Some(serde_json::to_value(body)?);
        Ok(self)
    }

    pub fn fetch_mode_no_cors(mut self) -> Self {
        self.fetch_mode_no_cors = true;
        self
    }
}

impl Default for APIRequestReqwest {
    fn default() -> Self {
        Self::new()
    }
}
