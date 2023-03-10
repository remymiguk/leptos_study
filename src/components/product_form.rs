use crate::components::modal::Confirmation;
use crate::states::app_state::read_global_state;
use crate::states::input_bind::*;
use crate::utils::navigator_back;
use crate::{
    components::hold_on::*,
    models::table_model::{Product, TableModel},
    states::{form_object::*, object_model::ObjectModelBuilder},
};
use leptos::*;
use leptos_router::use_params_map;
use uuid::Uuid;

#[component]
pub fn ProductForm(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let model = read_global_state::<TableModel<Product>>(cx);

    let product = create_local_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        move |id| {
            let model = model.clone();
            async move {
                if id.is_empty() {
                    None
                } else {
                    // @@@ EXPORT this read as signal

                    model
                        .clone()
                        .read(cx, id.parse::<Uuid>().unwrap())
                        .await
                        .map_err(|e| error!("{e}"))
                        .ok()
                        .flatten()
                }
            }
        },
    );

    view! {
        cx,
        <HoldOnRes
            resource=move || product
            fallback=move |cx|view! { cx, "Loading..." }.into_view(cx)
            error=move |cx|view! { cx, <div class="item-view">"Error loading this product."</div> }.into_view(cx)
            child=move |cx, product| view! { cx, <LoadedProductForm product/> }.into_view(cx)
        />
    }
}

#[component]
pub fn LoadedProductForm(cx: Scope, product: Product) -> impl IntoView {
    let table_model = read_global_state::<TableModel<Product>>(cx);

    let object_model = ObjectModelBuilder::new(cx, product).build();

    let object_read = object_model.object_read;

    let fo = FormObject::new(object_model);

    let confirm_cancel = Confirmation::new(cx);
    let confirm_ok = Confirmation::new(cx);

    let on_save = move |_| {
        let (saved_read, saved_write) = create_signal(cx, Option::<Result<(), String>>::None);

        let _ = create_memo(cx, move |_| {
            if let Some(Ok(_)) = saved_read() {
                navigator_back();
            }
        });

        table_model
            .update_write
            .set(Some((saved_write, object_read())));
    };

    view! { cx,
        <div>
            {confirm_cancel.component(cx, "Confirm cancellation?", move |_| navigator_back())}
            {confirm_ok.component(cx, "Confirm saving?", on_save)}

            <div>{move ||format!("Object content: {:?}", object_read())}</div>

            <InputBind fo=&fo input_type="uuid" literal="Id" field_name="id" placeholder="Id"/>
            <InputBind fo=&fo input_type="text" literal="Description" field_name="description" placeholder="Description"/>
            <InputBind fo=&fo input_type="decimal" literal="Price" field_name="price" placeholder="Price"/>
            <br/>
            <br/>

            <input class="button is-danger" on:click=confirm_cancel.on_show() type="button" value="Cancel"/>
            <input class="button is-success" on:click=confirm_ok.on_show() type="button" value="Ok"/>
        </div>
    }
}
