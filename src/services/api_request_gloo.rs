use crate::app::error::{AppError, ErrorInfo};
use gloo::utils::window;
use gloo_net::http::{Method, Request};
use log::info;
use serde::{de::DeserializeOwned, Serialize};
use web_sys::{RequestMode, RequestRedirect};

pub const API_ROOT: &str = "http://127.0.0.1:3000";

// TODO: create
pub struct APIRequestGloo {
    token: Option<String>,
    fetch_mode_no_cors: bool,
    allow_redirect: bool,
    body: Option<serde_json::Value>,
}

impl APIRequestGloo {
    pub fn new() -> Self {
        Self {
            token: None,
            fetch_mode_no_cors: false,
            allow_redirect: true,
            body: None,
        }
    }

    /// build all kinds of http request: post/get/delete etc.
    pub async fn request<T>(&self, method: Method, url: &str) -> Result<T, AppError>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
    {
        info!("### url: {}...", url);

        let allow_body = matches!(method, Method::POST | Method::PUT | Method::PATCH);

        let url = format!("{API_ROOT}/{url}");
        let mut builder = Request::new(&url)
            .method(method)
            .header("Content-Type", "application/json");

        if self.fetch_mode_no_cors {
            builder = builder.mode(RequestMode::NoCors);
        }

        if !self.allow_redirect {
            builder = builder.redirect(RequestRedirect::Manual);
        }

        if let Some(token) = self.token.as_ref() {
            let header_value = format!("Bearer {token}");
            builder = builder.header("authorization", &header_value);

            let cookie = format!(
                "{}={}; SameSite=Lax; Path=/; domain=.localhost.org",
                "SESSION", token
            );
            builder = builder.header("cookie", &cookie);
            info!("defined cookie");
        }

        if allow_body {
            builder = builder.json(&self.body)?;
        }

        let response = builder.send().await;

        info!("### url response: {:?}", response);

        if let Ok(response) = response {
            let status = response.status();
            info!("### url status: {status}");
            if response.ok() {
                let future = response.json::<T>();
                let data: Result<T, _> = future.await;
                match data {
                    Ok(data) => {
                        log::debug!("Response: {:?}", data);
                        Ok(data)
                    }
                    Err(e) => Err(e.into()),
                }
            } else {
                let status = response.status();
                match status {
                    302 | 303 | 307 => {
                        let location = response.headers().get("location").unwrap_or_default();
                        Err(AppError::Redirect(location))
                    }
                    400 if self.allow_redirect => {
                        let body = response.text().await.unwrap();
                        if !body.is_empty() {
                            let url = format!("{API_ROOT}{body}");
                            window().location().set_href(&url).ok();
                        }
                        Err(AppError::Redirect(body))
                    }
                    401 => Err(AppError::Unauthorized),
                    403 => Err(AppError::Forbidden),
                    404 => Err(AppError::NotFound),
                    500 => Err(AppError::InternalServerError),
                    422 => {
                        let future = response.json::<ErrorInfo>();
                        let data: Result<ErrorInfo, gloo_net::Error> = future.await;
                        match data {
                            Ok(data) => Err(AppError::UnprocessableEntity(data)),
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

    pub fn allow_redirect(mut self, allow_redirect: bool) -> Self {
        self.allow_redirect = allow_redirect;
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

impl Default for APIRequestGloo {
    fn default() -> Self {
        Self::new()
    }
}
