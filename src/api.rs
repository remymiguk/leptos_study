use crate::app::error::AppError;
use leptos::{Scope, Serializable};

#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<T>(cx: Scope, path: &str) -> Result<Option<T>, AppError>
where
    T: Serializable,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    let text = gloo_net::http::Request::get(path)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await?
        .text()
        .await?;
    if text.is_empty() {
        return Ok(None);
    }

    // abort in-flight requests if the Scope is disposed
    // i.e., if we've navigated away from this page
    leptos::on_cleanup(cx, move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });
    Ok(Some(T::from_json(&text)?))
}

#[cfg(feature = "ssr")]
pub async fn fetch_api<T>(_cx: Scope, path: &str) -> Result<Option<T>, AppError>
where
    T: Serializable,
{
    let client = reqwest::Client::new();
    let request = client.get(path).fetch_mode_no_cors().build().unwrap();
    let text = client.execute(request).await?.text().await?;
    if text.is_empty() {
        return Ok(None);
    }
    Ok(Some(T::from_json(&text)?))
}
