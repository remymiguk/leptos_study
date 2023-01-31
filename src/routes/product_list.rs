use crate::app::pagination::*;
use crate::app::repository::product_repository;
use crate::components::hold_on::*;
use crate::components::pagination::*;
use crate::models::product::Product;
use crate::models::product::ProductModel;
use crate::states::app_state::StateGetter;
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

#[component]
pub fn ProductList(cx: Scope) -> impl IntoView {
    let model = ProductModel::new(cx);

    // { ******** old
    // let query = use_query_map(cx);

    // let offset = move || {
    //     query.with(|q| {
    //         let offset = q
    //             .get("offset")
    //             .and_then(|offset| offset.parse::<usize>().ok())
    //             .unwrap_or(0);
    //         info!("offset: {offset}");
    //         offset
    //     })
    // };

    // let query = use_query_map(cx);
    // let limit = move || {
    //     query.with(|q| {
    //         let limit = q
    //             .get("limit")
    //             .and_then(|limit| limit.parse::<usize>().ok())
    //             .unwrap_or(10);
    //         info!("limit: {limit}");
    //         limit
    //     })
    // };
    //
    // } ******** old

    let page_write = model.page_write;
    let list_reader = model.list_reader;
    let max_page = model.max_page;
    let page_read = model.page_read;

    let on_page_click = move |page: usize| page_write.set(page);

    // let (model_read, _) = create_signal(cx, model);
    // provide_context(cx, StateGetter(model_read));

    // let (model_read, _) = create_signal(cx, model);
    provide_context(cx, model);

    view! {
        cx,
        <HoldOnCx
            read={list_reader}
            fallback={move |cx|view! { cx, "Loading..." }.into_view(cx)}
            error={move |cx|view! { cx, <div class="item-view">"Error loading this product."</div> }.into_view(cx)}
            child={move |cx, (products, count)| view! {
                cx,
                <LoadedProducts products count/>
                <Pagination max=max_page() current=page_read() on_page_click/>
            }.into_view(cx)}
        />
    }
}

#[component]
pub fn LoadedProducts(cx: Scope, products: Vec<Product>, count: usize) -> impl IntoView {
    view! { cx,
        <h3 class="title is-4">{ format!("Products list {count}") }</h3>
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
            <A href=format!("/product/{}", product.id)>
                { product.description.clone() }
            </A>
        </div>
    }
}
