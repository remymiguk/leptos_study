use crate::components::modal::Confirmation;
use crate::utils::navigator_back;
use crate::{
    components::hold_on::*,
    models::product::{Product, ProductModel},
    states::{form_object::*, object_model::ObjectModelBuilder},
};
use leptos::*;
use leptos_router::use_params_map;
use uuid::Uuid;

#[component]
pub fn ProductForm(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    //let model = use_context::<StateGetter<ProductModel>>(cx).unwrap().0();
    let model = use_context::<Option<ProductModel>>(cx).unwrap().unwrap();

    let product = create_local_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        move |id| {
            let model = model.clone();
            async move {
                if id.is_empty() {
                    None
                } else {
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
        <HoldOn
            read=move || product.read()
            fallback=move ||view! { cx, "Loading..." }.into_view(cx)
            error=move ||view! { cx, <div class="item-view">"Error loading this product."</div> }.into_view(cx)
            child=move |product| view! { cx, <LoadedProductForm product/> }.into_view(cx)
        />
    }
}

#[component]
pub fn LoadedProductForm(cx: Scope, product: Product) -> impl IntoView {
    let model = ObjectModelBuilder::new(cx, product).build();
    let fo = FormObject::new(model);

    let confirm_cancel = Confirmation::new(cx);
    let confirm_ok = Confirmation::new(cx);

    view! { cx,
        <div>
            {confirm_cancel.component(cx, "Confirm cancellation?", move |_| navigator_back())}
            {confirm_ok.component(cx, "Confirm saving?", move |_| navigator_back())}


            <InputBind fo=&fo input_type="text" literal="Id" field_name="id" placeholder="Id"/>
            <InputBind fo=&fo input_type="text" literal="Description" field_name="description" placeholder="Description"/>
            <InputBind fo=&fo input_type="text" literal="Price" field_name="price" placeholder="Price"/>
            <br/>
            <br/>

            <input class="button is-danger" on:click=confirm_cancel.on_show() type="button" value="Cancel"/>
            <input class="button is-success" on:click=confirm_ok.on_show() type="button" value="Ok"/>
        </div>
    }
}
