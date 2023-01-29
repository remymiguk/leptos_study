use leptos::window;

pub fn navigator_back() {
    let navigator = window().history().unwrap();
    navigator.back().unwrap();
}
