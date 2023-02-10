use crate::states::form_object::FormObject;
use crate::states::object::Object;
use leptos::*;
use voxi_core::ValueType;

#[component]
pub fn CheckboxBind<T, 'a>(
    cx: Scope,
    fo: &'a FormObject<T>,
    #[prop(into)] literal: String,
    #[prop(into)] field_name: String,
) -> impl IntoView
where
    T: Object,
{
    let onchange = fo.on_change_checked_to_map(field_name.clone(), ValueType::Boolean);

    let value = fo.memo_content(cx, field_name.clone(), ValueType::Boolean);
    let checked = move || value().0 == "true";

    let is_valid_signal = fo.memo_valid(cx, field_name.clone());
    let hint_signal = fo.memo_hint(cx, field_name);

    let is_success_read = create_memo(cx, move |_| {
        if is_valid_signal() {
            ("is-success", "fa-check")
        } else {
            ("is-danger", "fa-exclamation-triangle")
        }
    });

    let hint_bottom = create_memo(cx, move |_| {
        hint_signal().map(|hint| {
            let is_success = is_success_read().0;
            view! {
                cx,
                <p class={format!("help {is_success}")}>{hint}</p>
            }
            .into_view(cx)
        })
    });

    let checkbox = view! {
        cx,
        <label class="checkbox">
            <input type="checkbox" on:change=onchange checked=checked/>
            { " " }{ literal }
        </label>
    };

    view! {
        cx,
        <div class="field">
            <div class="control has-icons-left has-icons-right">
                { checkbox }
                // { icon_left }
                // { icon_right }
            </div>
            { hint_bottom }
        </div>

    }
}
