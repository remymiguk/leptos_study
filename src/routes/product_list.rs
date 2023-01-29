use crate::app::pagination::*;
use crate::app::repository::product_repository;
use crate::components::hold_on::*;
use crate::components::pagination::*;
use crate::models::product::Product;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ProductList(cx: Scope) -> impl IntoView {
    let (page_read, page_write) = create_signal(cx, 1);

    let offset_read = move || page_read() - 1;

    let (limit_read, limit_write) = create_signal(cx, 3);

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

    let count = create_local_resource(
        cx,
        || {},
        move |_| async move {
            product_repository()
                .count()
                .await
                .map_err(|e| error!("{e}"))
        },
    );

    let products = create_local_resource(
        cx,
        move || (offset_read(), limit_read()),
        move |(offset, limit)| async move {
            product_repository()
                .list(cx, Limit(limit), Offset(offset))
                .await
                .map_err(|e| error!("{e}"))
        },
    );

    // Calc max page
    let max_page = move || match count.read() {
        Some(Ok(count)) => (count as f32 / limit_read() as f32).ceil() as usize,
        _ => 0,
    };

    let list_reader = create_memo(cx, move |_| match (products.read(), count.read()) {
        (Some(Ok(products)), Some(Ok(count))) => Some(Some((products, count))),
        (Some(Err(_)), _) | (_, Some(Err(_))) => Some(None),
        _ => None,
    });

    let on_page_click = move |page: usize| page_write.set(page);

    view! {
        cx,
        <HoldOn
            read={list_reader}
            fallback={move ||view! { cx, "Loading..." }.into_view(cx)}
            error={move ||view! { cx, <div class="item-view">"Error loading this product."</div> }.into_view(cx)}
            child={move |(products, count)| view! {
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
