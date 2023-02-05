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
    let onchange = fo.on_change_to_map(field_name.clone(), ValueType::Boolean);

    let value = fo.memo_content(cx, field_name, ValueType::Boolean);
    let checked = move || value().0 == "true";

    let checkbox = view! {
        cx,
        <label class="checkbox">
            <input type="checkbox" on:change={onchange} checked={checked}/>
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
            // { hint_bottom }
        </div>

    }
}
