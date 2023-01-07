use crate::{api, models::product::Product};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn ProductForm(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let product = create_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                api::fetch_api::<Product>(cx, &api::products(&format!("/{id}"))).await
            }
        },
    );
    let meta_description = move || {
        product
            .read()
            .and_then(|product| product.map(|product| product.description.clone()))
            .unwrap_or_else(|| "Loading story...".to_string())
    };

    view! {
        cx,
        <>
            <Meta name="description" content=meta_description/>
            <Suspense fallback=|| view! { cx, "Loading..." }>
                {move || product.read().map(|product| match product {
                    None => view! { cx,  <div class="item-view">"Error loading this product."</div> }.into_view(cx),
                    Some(product) => view! { cx, <LoadedProductForm product /> }.into_view(cx),
                    })
                }
            </Suspense>
        </>


    }
}

#[component]
pub fn LoadedProductForm(cx: Scope, product: Product) -> impl IntoView {
    let on_click = move |_| {
        let navigator = window().history().unwrap();
        navigator.back().unwrap();
    };

    view! { cx,
        <div>
            <div>{ "id" }</div>
            <div>{product.id.to_string()}</div>
            <div>{ "Description" }</div>
            <div> {product.description}</div>
            <div>{ "Price" }</div>
            <div>{product.price.to_string()}</div>
            <input
                on:click=on_click
                type="button"
                value="Cancel"/>
        </div>
    }
}
