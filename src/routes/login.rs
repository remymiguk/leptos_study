use crate::states::{
    app_state::{AppState, LoggedUser, StateSetter},
    form_object::*,
    object_model::ObjectModel,
    validator::{ValidatorPassword, ValidatorProvider},
};
use leptos::*;
use serde::{Deserialize, Serialize};
use voxi_core::ValueType;

pub fn apply_login(cx: Scope, email_password: EmailPassword) {
    let set_app_state = use_context::<StateSetter<AppState>>(cx).unwrap().0;

    set_app_state.update(move |app_state| {
        *app_state = app_state.clone().with_login(LoggedUser {
            name: email_password.email.clone().unwrap_or_default(),
            email: email_password.email.unwrap_or_default(),
        })
    });
}

// Where is the typing event?
// Where are the fields are being cleaned?

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct EmailPassword {
    email: Option<String>,
    password: Option<String>,
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let validators = vec![Box::new(
        ValidatorPassword::new(ValueType::String, "password").add_input(ValueType::String, "email"),
    ) as Box<dyn ValidatorProvider + 'static + Send + Sync>];

    let model = ObjectModel::new(cx, EmailPassword::default(), validators);

    let fo = FormObject::new(model.clone());

    let (read_signal, write_signal) = model.signal();
    view! {
        cx,
            <div>{move ||format!("Object content: {:?}", read_signal())}</div>
            <InputBind fo=&fo literal="E-mail" field_name="email" placeholder="User e-mail"/>
            <InputBind fo=&fo literal="Password" field_name="password" placeholder="User password"/>
            <br/><br/>
            <input type="button" value="Login" class="button is-danger"
                on:click=move |_| {
                    apply_login(cx, read_signal());
                    history_back();
                }
            />
            <br/><br/>
            <input type="button" value="Clear" class="button is-danger"
                on:click=move |_| write_signal.set(EmailPassword::default().into())
            />

    }
}

fn history_back() {
    window().history().unwrap().back().unwrap()
}
