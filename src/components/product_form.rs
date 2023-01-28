use crate::{
    models::product::{Product, ProductModel},
    states::{
        app_state::StateGetter,
        form_object::*,
        object_model::{ObjectModel, ObjectModelBuilder},
    },
};
use leptos::*;
use leptos_meta::*;
use leptos_router::use_params_map;
use log::info;
use uuid::Uuid;

#[component]
pub fn ProductFormOri(cx: Scope) -> impl IntoView {
    info!("##################");

    let params = use_params_map(cx);
    let model = use_context::<StateGetter<ProductModel>>(cx).unwrap().0();

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
                None => view! { cx, <div class="item-view">"Error loading this product."</div> }.into_view(cx),
                Some(product) =>
                    view! { cx, <LoadedProductForm product/> }.into_view(cx)
                })
            }
        </Suspense>
    }
}

#[component]
pub fn ProductForm(cx: Scope) -> impl IntoView {
    info!("##################");

    let params = use_params_map(cx);
    let model = use_context::<StateGetter<ProductModel>>(cx).unwrap().0();

    let product = create_local_resource(
        cx,
        move || {},
        move |_| {
            Some(Product {
                id: Uuid,
                description: String,
                category: Uuid,
                price: Decimal,
                created_at: NaiveDateTime,
            })
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
                None => view! { cx, <div class="item-view">"Error loading this product."</div> }.into_view(cx),
                Some(product) =>
                    view! { cx, <LoadedProductForm product/> }.into_view(cx)
                })
            }
        </Suspense>
    }
}

#[component]
pub fn LoadedProductForm(cx: Scope, product: Product) -> impl IntoView {
    info!("***************");

    let (read_signal, _) = create_signal(cx, product);

    let read_resource = create_local_resource(cx, read_signal, |arg| async move { arg });

    let _ = create_memo(cx, move |_| read_resource.read());

    view! { cx,
        <div>
            <input class="button is-danger" type="button" value="Cancel"/>
        </div>
    }
}

#[component]
pub fn LoadedProductFormOri(cx: Scope, product: Product) -> impl IntoView {
    info!("***************");

    // let model = ObjectModelBuilder::new(cx, product).build();
    // let fo = FormObject::new(model);

    let (public_to_validate, _public_object_writer) = create_signal(cx, product);

    let diff_validated_reader =
        create_local_resource(cx, public_to_validate, |json_changed| async move {
            json_changed
        });

    let _public_component_reader = create_memo(cx, move |_| diff_validated_reader.read());

    view! { cx,
        <div>
            // <InputBind fo=&fo input_type="text" literal="Id" field_name="id" placeholder="Id"/>
            // <InputBind fo=&fo input_type="text" literal="Description" field_name="description" placeholder="Description"/>
            // <InputBind fo=&fo input_type="text" literal="Price" field_name="price" placeholder="Price"/>
            <br/>
            <br/>
            <input
                class="button is-danger"
                on:click=move |_| navigator_back()
                type="button"
                value="Cancel"/>
        </div>
    }
}

fn navigator_back() {
    let navigator = window().history().unwrap();
    navigator.back().unwrap();
}
