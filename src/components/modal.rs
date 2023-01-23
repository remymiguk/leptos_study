use leptos::*;
use leptos_router::*;
use log::info;
use web_sys::MouseEvent;

pub struct Confirmation {
    is_active_write: WriteSignal<bool>,
    is_active_read: ReadSignal<bool>,
}

impl Confirmation {
    pub fn new(cx: Scope) -> Self {
        let (is_active_read, is_active_write) = create_signal(cx, false);
        Self {
            is_active_write,
            is_active_read,
        }
    }

    pub fn component<F>(&self, cx: Scope, message: &str, on_confirm: F) -> impl IntoView
    where
        F: Fn(MouseEvent) + 'static,
    {
        let is_active_write = self.is_active_write;
        let is_active_read = self.is_active_read;
        let title = "Confirmation";
        let confirmation = true;
        view! {
            cx,
            <Modal
                title
                message
                is_active_write
                is_active_read
                on_confirm
                confirmation
            />
        }
    }

    pub fn show(&self) {
        self.is_active_write.set(true);
    }

    pub fn on_show(&self) -> impl Fn(MouseEvent) {
        let is_active_write = self.is_active_write;
        move |_| {
            info!("*** inside on_show set true");
            is_active_write.set(true)
        }
    }
}

#[component]
pub fn Modal<F>(
    cx: Scope,
    #[prop(into)] title: String,
    #[prop(into)] message: String,
    is_active_write: WriteSignal<bool>,
    is_active_read: ReadSignal<bool>,
    on_confirm: F,
    confirmation: bool,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let is_active_s = move || {
        if is_active_read() {
            info!("*** modal is-active");
            "modal is-active"
        } else {
            "modal"
        }
    };

    let cancel_title = if confirmation { "Cancel" } else { "Ok" };

    let on_click_cancel = move |_| is_active_write.set(false);

    let on_click_ok = move |event| {
        is_active_write.set(false);
        on_confirm(event);
    };

    let confirm_button = if confirmation {
        Some(view! {
            cx,
            <button class="button" on:click=on_click_ok>
                "Ok"
            </button>
        })
    } else {
        None
    };

    view! {
        cx,
        <div class=is_active_s>
            <div class="modal-background"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                <p class="modal-card-title">{title}</p>
                <button class="delete" aria-label="close"
                    on:click=on_click_cancel>
                </button>
                </header>
                <section class="modal-card-body">
                    {message}
                </section>
                <footer class="modal-card-foot">
                    // Cancel button
                    <button class="button" on:click=on_click_cancel>
                        {cancel_title}
                    </button>
                    // Confirm button
                    {confirm_button}
                </footer>
            </div>
        </div>
    }
    .into_view(cx)
}
