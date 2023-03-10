use crate::components::hold_on::*;
use crate::components::pagination::*;
use crate::models::table_model::Product;
use crate::models::table_model::TableModel;
use crate::states::app_state::read_global_state;
use leptos::*;

#[component]
pub fn ProductTable(cx: Scope) -> impl IntoView {
    let table_model = read_global_state::<TableModel<Product>>(cx);

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

    let page_write = table_model.page_write;
    let list_reader = table_model.list_reader;
    let max_page = table_model.max_page;
    let page_read = table_model.page_read;

    let on_page_click = move |page: usize| page_write.set(page);

    view! {
        cx,
        <HoldOn
            read=list_reader
            fallback=move |cx|view! { cx, "Loading..." }.into_view(cx)
            error=move |cx|view! { cx, <div class="item-view">"Error loading this product."</div> }.into_view(cx)
            child=move |cx, (products, count)| view! {
                cx,
                <LoadedProducts products count/>
                <Pagination max=max_page() current=page_read() on_page_click/>
            }.into_view(cx)
        />
    }
}

#[component]
pub fn LoadedProducts(cx: Scope, products: Vec<Product>, count: usize) -> impl IntoView {
    view! { cx,
        <h3 class="title is-4">{ format!("Products list {count}") }</h3>
        <ul>
            {products.into_iter().map(|product|view! { cx, <ProductRow product/>}).collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
pub fn ProductRow(cx: Scope, product: Product) -> impl IntoView {
    view! {
        cx,
        <div>
            <a href=format!("/product/{}", product.id)>
                { product.description }
            </a>
        </div>
    }
}
