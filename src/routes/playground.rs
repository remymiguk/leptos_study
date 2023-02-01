use leptos::*;
use log::info;

#[component]
pub fn Playground(cx: Scope) -> impl IntoView {
    let (signal_read, signal_write) = create_signal(cx, String::from("..."));

    let _ = create_memo(cx, move |_| {
        let s = signal_read() + "xxx";
        info!("*** inside memo");
        s
    });

    view! {
        cx,
            <h4>
                {move || signal_read.get()}
            </h4>
            // <h4>
            //     {move || upsert_value}
            // </h4>
            // <br/><br/>
            // <input type="button" value="Signal" class="button is-danger"
            //     on:click=on_signal
            // />
            // <input type="button" value="Action" class="button is-danger"
            //     on:click=on_action
            // />
    }
}

async fn switch(payload: String) -> String {
    if payload == "..." {
        String::from("xxx")
    } else {
        String::from("...")
    }
}
