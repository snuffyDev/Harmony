pub mod client;
pub use client::{HttpClient, HttpOptions, HttpPostOptions, HttpResponse};
use surf::http::Method;

#[tauri::command]
pub async fn post(opts: HttpPostOptions) -> Result<HttpResponse, String> {
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
