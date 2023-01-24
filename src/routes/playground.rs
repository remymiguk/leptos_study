use leptos::*;
use log::info;

#[component]
pub fn Playground(cx: Scope) -> impl IntoView {
    let (signal_read, signal_write) = create_signal(cx, String::from("..."));

    let on_signal = move |_| {
        signal_write.update(|payload| {
            if payload == "..." {
                *payload = "xxx".into()
            } else {
                *payload = "...".into()
            }
        });
    };

    let upsert = create_action(cx, |payload: &String| switch(payload.to_string()));

    let upsert_value = upsert.value();
    let on_action = move |_| {
        let upsert_value = upsert_value.clone();
        let value_opt = upsert_value();
        let value = if Some(String::from("...")) == value_opt {
            String::from("xxx")
        } else {
            String::from("...")
        };
        info!("dispatching {value}");
        upsert.dispatch(value);
    };

    view! {
        cx,
            <h4>
                {move || signal_read.get()}
            </h4>
            <h4>
                {move || upsert_value}
            </h4>
            <br/><br/>
            <input type="button" value="Signal" class="button is-danger"
                on:click=on_signal
            />
            <input type="button" value="Action" class="button is-danger"
                on:click=on_action
            />
    }
}

async fn switch(payload: String) -> String {
    if payload == "..." {
        String::from("xxx")
    } else {
        String::from("...")
    }
}
