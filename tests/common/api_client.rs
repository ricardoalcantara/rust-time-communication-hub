use reqwest::{Method, RequestBuilder, Response, StatusCode};
use reqwest_eventsource::EventSource;
use rust_time_communication_hub::domain::{
    auth::dto::auth::AccessToken, dto::error::ErrorResponse,
};
use std::marker::PhantomData;

pub struct ApiResponse<T>
where
    T: serde::de::DeserializeOwned,
{
    response: Response,
    _marker: PhantomData<*const T>,
}

impl<T> ApiResponse<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    pub async fn json_error(self) -> ErrorResponse<String> {
        self.response.json::<ErrorResponse<String>>().await.unwrap()
    }

    pub async fn json(self) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        self.response.json::<T>().await.unwrap()
    }
}

impl<T> From<Response> for ApiResponse<T>
where
    T: serde::de::DeserializeOwned,
{
    fn from(val: Response) -> Self {
        ApiResponse {
            response: val,
            _marker: PhantomData,
        }
    }
}

pub struct ApiClient {
    access_token: Option<AccessToken>,
    admin_url: String,
    sse_url: String,
}

impl ApiClient {
    pub fn new() -> ApiClient {
        let admin_url = std::env::var("TEST_ADMIN_URL")
            .unwrap_or_else(|_| String::from("http://localhost:4501"));
        let sse_url = std::env::var("TEST_ADMIN_URL")
            .unwrap_or_else(|_| String::from("http://localhost:4500"));
        ApiClient {
            access_token: None,
            admin_url,
            sse_url,
        }
    }

    pub fn access_token<T: Into<Option<AccessToken>>>(&mut self, access_token: T) {
        self.access_token = access_token.into();
    }

    pub fn new_with_url<S: Into<String>>(admin_url: S, sse_url: S) -> ApiClient {
        ApiClient {
            access_token: None,
            admin_url: admin_url.into(),
            sse_url: sse_url.into(),
        }
    }

    pub fn build_request<S: Into<String> + reqwest::IntoUrl>(
        &self,
        url: S,
        method: Method,
        with_credentials: bool,
    ) -> RequestBuilder {
        let mut request = reqwest::Client::new().request(method, url);
        if with_credentials {
            if let Some(access_token) = &self.access_token {
                request = request.header(
                    "Authorization",
                    format!("Bearer {}", access_token.access_token),
                );
            }
        }

        request
    }

    pub async fn get<T, S: Into<String> + reqwest::IntoUrl>(
        &self,
        url: S,
        with_credentials: bool,
    ) -> ApiResponse<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.build_request(url, Method::GET, with_credentials)
            .send()
            .await
            .unwrap()
            .into()
    }

    pub async fn post<T, D, S: Into<String> + reqwest::IntoUrl>(
        &self,
        url: S,
        data: &D,
        with_credentials: bool,
    ) -> ApiResponse<T>
    where
        D: serde::ser::Serialize,
        T: serde::de::DeserializeOwned,
    {
        self.build_request(url, Method::POST, with_credentials)
            .json(data)
            .send()
            .await
            .unwrap()
            .into()
    }

    pub async fn delete<T, D, S: Into<String> + reqwest::IntoUrl>(
        &self,
        url: S,
        data: &D,
        with_credentials: bool,
    ) -> ApiResponse<T>
    where
        D: serde::ser::Serialize,
        T: serde::de::DeserializeOwned,
    {
        self.build_request(url, Method::DELETE, with_credentials)
            .json(data)
            .send()
            .await
            .unwrap()
            .into()
    }

    pub async fn put<T, D, S: Into<String> + reqwest::IntoUrl>(
        &self,
        url: S,
        data: &D,
        with_credentials: bool,
    ) -> ApiResponse<T>
    where
        D: serde::ser::Serialize,
        T: serde::de::DeserializeOwned,
    {
        self.build_request(url, Method::PUT, with_credentials)
            .json(data)
            .send()
            .await
            .unwrap()
            .into()
    }
}

impl ApiClient {
    pub async fn auth_token(&self) -> ApiResponse<AccessToken> {
        self.get(format!("{}/api/auth/token", self.admin_url), false)
            .await
    }
}

impl ApiClient {
    pub fn get_event_source<S: Into<String>>(&self, access_token: S) -> EventSource {
        EventSource::get(format!(
            "{}/sse?token={}",
            self.sse_url,
            access_token.into()
        ))
    }
}
