use crate::api::{self, products};
use crate::models::product::Product;
use leptos::*;
// use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Products(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);

    let offset = move || {
        query.with(|q| {
            q.get("offset")
                .and_then(|offset| offset.parse::<usize>().ok())
                .unwrap_or(0)
        })
    };

    let limit = move || {
        query.with(|q| {
            q.get("limit")
                .and_then(|limit| limit.parse::<usize>().ok())
                .unwrap_or(10)
        })
    };

    let products = create_resource(
        cx,
        move || (offset(), limit()),
        move |(offset, limit)| async move {
            let path = format!("?offset={offset}&limit={limit}");
            api::fetch_api::<Vec<Product>>(cx, &products(&path)).await
        },
    );

    view! {
        cx,

        <Suspense fallback=|| view! { cx, "Loading..." }>
            {move || match products.read() {
                None => None,
                Some(None) => Some(view! { cx,  <p>"Error loading products."</p> }.into_view(cx)),
                Some(Some(products)) =>
                    Some(view! { cx, <LoadedProducts products /> }.into_view(cx)),

            }}
        </Suspense>

    }
}

#[component]
pub fn LoadedProducts(cx: Scope, products: Vec<Product>) -> impl IntoView {
    view! { cx,
        <ul>
            <For
                each=move || products.clone()
                key=|product| product.id
                view=move |product: Product| {
                    view! { cx,
                        <ProductRow product/>
                    }
                }
            />
        </ul>
    }
}

#[component]
pub fn ProductRow(cx: Scope, product: Product) -> impl IntoView {
    view! {
        cx,
        <div>
            <a href=format!("/products/{}", product.id)>
                { product.description }
            </a>
        </div>
    }
}
