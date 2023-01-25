use crate::{
    models::product::{Product, ProductModel},
    states::{app_state::StateGetter, form_object::*, object_model::ObjectModel},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;
use uuid::Uuid;

#[component]
pub fn ProductForm(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let product = create_local_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                let model = use_context::<StateGetter<ProductModel>>(cx).unwrap().0();
                model
                    .clone()
                    .read(cx, id.parse::<Uuid>().unwrap())
                    .await
                    .map_err(|e| error!("{e}"))
                    .ok()
                    .flatten()
            }
        },
    );

    let meta_description = move || {
        product
            .read()
            .and_then(|product| product.map(|product| product.description))
            .unwrap_or_else(|| "Loading product...".to_string())
    };

    view! {
        cx,
        <Meta name="description" content=meta_description/>
        <Suspense fallback=|| view! { cx, "Loading..." }>
            {move || product.read().map(|product| match product {
                None => view! { cx,  <div class="item-view">"Error loading this product."</div> }.into_view(cx),
                Some(product) => view! { cx, <LoadedProductForm product /> }.into_view(cx),
                })
            }
        </Suspense>
    }
}

#[component]
pub fn LoadedProductForm(cx: Scope, product: Product) -> impl IntoView {
    let on_click = move |_| {
        let navigator = window().history().unwrap();
        navigator.back().unwrap();
    };

    let model = ObjectModel::new(cx, product, vec![]);

    let fo = FormObject::new(model);

    view! { cx,
        <div>
            <div>{ "id" }</div>

            <InputBind fo=&fo input_type="text" literal="Id" field_name="id" placeholder="Id"/>

            <InputBind fo=&fo input_type="text" literal="Description" field_name="description" placeholder="Description"/>

            <InputBind fo=&fo input_type="text" literal="Price" field_name="price" placeholder="Price"/>

            <br/>
            <br/>
            <input
                class="button is-danger"
                on:click=on_click
                type="button"
                value="Cancel"/>
        </div>
    }
}
