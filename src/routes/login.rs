use crate::components::modal::Confirmation;
use crate::states::{
    app_state::{AppState, LoggedUser, StateSetter},
    email_validator::ValidatorEmail,
    form_object::*,
    object_model::ObjectModel,
    password_validator::ValidatorPassword,
    validator::Validators,
};
use leptos::*;
use serde::{Deserialize, Serialize};

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
    let validators = Validators::new()
        .add(ValidatorPassword::new())
        .add(ValidatorEmail::new());

    let model = ObjectModel::new(cx, EmailPassword::default(), validators);

    let (read_signal, write_signal) = model.signal();

    let fo = FormObject::new(model);

    let confirm_clear = Confirmation::new(cx);

    view! {
        cx,
            {confirm_clear.component(cx, "Confirm clear?", move |_| write_signal.set(EmailPassword::default()))}

            <div>{move ||format!("Object content: {:?}", read_signal())}</div>

            <InputBind fo=&fo input_type="text" literal="E-mail" field_name="email" placeholder="User e-mail"/>
            <InputBind fo=&fo input_type="password" literal="Password" field_name="password" placeholder="User password"/>

            <br/><br/>
            <input type="button" value="Login" class="button is-danger"
                on:click=move |_| {
                    apply_login(cx, read_signal());
                    history_back();
                }
            />
            <br/><br/>
            <input type="button" value="Clear" class="button is-danger"
                on:click=confirm_clear.on_show()
            />

    }
}

fn history_back() {
    window().history().unwrap().back().unwrap()
}
