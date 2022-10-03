use crate::utils::timer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use surf::http::{Method, Url};
use surf::{Client, Response, StatusCode};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct ParseResult {
    body: Option<Vec<u8>>,
    status_code: StatusCode,
    content_type: String,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct HttpResponse {
    body: Option<Vec<u8>>,
    status_code: StatusCode,
    content_type: String,
    response_time: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct HttpClient {
    client: Arc<Mutex<Client>>,
    method: Method,
    url: Url,
    body: Option<Vec<u8>>,
    follow_redirects: Option<bool>,
}

// Parse the http response
pub async fn parse_response(mut response: Response) -> ParseResult {
    let body = if let Ok(body) = response.body_bytes().await {
        Some(body)
    } else {
        None
    };
    let status_code = response.status();
    let content_type = response
        .content_type()
        .expect("Invalid Content-Type")
        .to_string();

    ParseResult {
        body,
        content_type,
        status_code,
    }
}

impl HttpClient {
    fn new() -> Self {
        let client = Arc::new(Mutex::new(Client::new()));
        Self {
            client,
            method: Method::Get,
            url: Url::parse("https://www.google.com").unwrap(),
            body: None,
            follow_redirects: Some(true),
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Url::parse(url).expect("Error parsing Url");
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn follow_redirects(mut self, follow_redirects: Option<bool>) -> Self {
        self.follow_redirects = follow_redirects;
        self
    }

    pub fn build(self) -> Result<HttpClient, String> {
        if self.url.has_host() {
            Ok(self)
        } else {
            Err("Could not build new HttpClient".to_string())
        }
    }

    pub async fn send(self) -> Result<HttpResponse, String> {
        let follow_redirects = self.follow_redirects;
        let client_guard = Arc::clone(&self.client);

        let thread = tokio::spawn(async move {
            let client = client_guard.lock().await;

            let mut stopwatch = timer::Stopwatch::new();

            // Build a new HTTP Request
            let mut request = client.request(self.method, self.url).build();

            // Conditionally adds redirect middleware to the request
            if follow_redirects == Some(true) {
                request.middleware(surf::middleware::Redirect::default());
            }

            stopwatch.start();

            if let Ok(response) = client.send(request).await {
                stopwatch.stop();

                let ParseResult {
                    body,
                    content_type,
                    status_code,
                } = parse_response(response).await;

                Ok(HttpResponse {
                    body,
                    content_type,
                    status_code,
                    response_time: Some(stopwatch.calc_ms()),
                })
            } else {
                Err("Error sending request".to_string())
            }
        })
        .await
        .map_err(|err| eprintln!("{}", err.to_string()))
        .unwrap();
        thread
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpOptions {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostOptions {
    url: String,
    body: Vec<u8>,
    headers: Option<HashMap<String, String>>,
    content_type: String,
}

#[tauri::command]
pub async fn post(opts: PostOptions) -> Result<HttpResponse, String> {
    let client = HttpClient::new()
        .url(&opts.url)
        .method(Method::Post)
        .body(opts.body)
        .build()
        .expect("Error building POST HttpClient");

    let res = client.send().await.expect("Error sending request");
    Ok(res)
}

#[tauri::command]
pub async fn get(url: &str, follow_redirects: bool) -> Result<HttpResponse, String> {
    let redirect = if follow_redirects == true {
        Some(true)
    } else {
        Some(false)
    };

    let client = HttpClient::new()
        .url(url)
        .method(Method::Get)
        .follow_redirects(redirect)
        .build()
        .expect("Error building HttpClient");

    let res = client.send().await.expect("Error sending request");
    Ok(res)
}
