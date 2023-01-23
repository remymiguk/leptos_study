use leptos::*;
use leptos_router::*;
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

    pub fn component<F>(&self, cx: Scope, on_confirm: F) -> impl IntoView
    where
        F: Fn(MouseEvent) + 'static,
    {
        let is_active_write = self.is_active_write;
        let is_active_read = self.is_active_read;
        view! {
            cx,
            <Modal
                is_active_write
                is_active_read
                on_confirm
            />
        }
    }

    pub fn show(&self) {
        self.is_active_write.set(true);
    }

    pub fn on_show(&self) -> impl Fn(MouseEvent) {
        let is_active_write = self.is_active_write;
        move |_| is_active_write.set(true)
    }
}

#[component]
pub fn Modal<F>(
    cx: Scope,
    is_active_write: WriteSignal<bool>,
    is_active_read: ReadSignal<bool>,
    on_confirm: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let is_active_s = move || {
        if is_active_read() {
            "is-active"
        } else {
            ""
        }
    };

    let title = "Confirmation";

    let cancel_title = "Cancel";

    let message = "Confirm operation?";

    let on_click_cancel = move |_| is_active_write.set(false);

    let on_click_ok = move |event| {
        is_active_write.set(false);
        on_confirm(event);
    };

    let confirm_button = view! {
        cx,
        <button class="button" on:click=on_click_ok>
            "Ok"
        </button>
    };

    view! {
        cx,
        <div prop:class="modal "{is_active_sx}>
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
                    { confirm_button }
                </footer>
            </div>
        </div>
    }
    .into_view(cx)
}
