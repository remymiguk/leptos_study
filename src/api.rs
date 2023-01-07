use leptos::{Scope, Serializable};

pub fn story(path: &str) -> String {
    format!("https://node-hnapi.herokuapp.com/{path}")
}

pub fn user(path: &str) -> String {
    format!("https://hacker-news.firebaseio.com/v0/user/{path}.json")
}

pub fn products(path: &str) -> String {
    format!("http://127.0.0.1:8080/api/products{path}")
}

#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<T>(cx: Scope, path: &str) -> Option<T>
where
    T: Serializable,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    let json = gloo_net::http::Request::get(path)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    // abort in-flight requests if the Scope is disposed
    // i.e., if we've navigated away from this page
    leptos::on_cleanup(cx, move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });
    T::from_json(&json).ok()
}

#[cfg(feature = "ssr")]
pub async fn fetch_api<T>(_cx: Scope, path: &str) -> Option<T>
where
    T: Serializable,
{
    use log::info;

    info!("fetching {path}");

    let client = reqwest::Client::new();
    let request = client.get(path).fetch_mode_no_cors().build().unwrap();

    //reqwest::c
    let json = client
        .execute(request)
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;
    T::from_json(&json).map_err(|e| log::error!("{e}")).ok()
}
