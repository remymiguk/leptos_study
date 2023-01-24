use crate::app::pagination::{Limit, Offset};
use crate::app::repository::product_repository;
use crate::models::product::Product;
use leptos::*;
use leptos_router::*;
use web_sys::MouseEvent;

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

    let count = create_resource(
        cx,
        || {},
        move |_| async move {
            product_repository()
                .count()
                .await
                .map_err(|e| error!("{e}"))
        },
    );

    // let upsert = create_action(cx, move |payload: &String| async {});
    // let ret = upsert.dispatch(String::from("data"));

    let products = create_resource(
        cx,
        move || (offset(), limit()),
        move |(offset, limit)| async move {
            product_repository()
                .list(cx, Limit(limit), Offset(offset))
                .await
                .map_err(|e| error!("{e}"))
        },
    );

    // Calc max page
    let max_page = || match count.read() {
        Some(Ok(count)) => Some((count as f32 / limit() as f32).ceil() as usize),
        _ => None,
    };

    // Current page max page
    let current_page = move || {
        if limit() == 0 {
            1
        } else {
            (offset() / limit()) + 1
        }
    };
    // let max_page = (request.count.unwrap_or_default() as f32 / limit_offset.limit as f32).ceil() as u32;
    // let current_page = limit_offset.page() as u32;

    view! {
        cx,
        <Suspense fallback=|| view! { cx, "Loading..." }>
            {move || match (products.read(), count.read()) {
                (None, None) => None,
                (Some(Ok(products)), Some(Ok(count))) =>
                    Some(view! { cx, <LoadedProducts products count /> }.into_view(cx)),
                (_ ,_) => Some(view! { cx,  <p>{"Error loading products"}</p> }.into_view(cx)),
            }}
        </Suspense>

    }
}

#[component]
pub fn Pagination<F>(cx: Scope, max: usize, current: usize, on_page_click: F) -> impl IntoView
where
    F: Fn(usize) + Copy + 'static,
{
    let mut lis = vec![];

    let mut prev_visible = false;
    for page in 1..=max {
        let visible = page == 1 || page == max || (page as i32 - current as i32).abs() <= 1;
        let li = if visible {
            let is_current = page == current;
            view! {cx, <Page number={page} is_current={is_current} on_click_number={on_page_click} /> }.into_view(cx)
        } else {
            if !prev_visible {
                continue;
            }
            view! { cx, <Ellipsis /> }.into_view(cx)
        };
        prev_visible = visible;
        lis.push(li);
    }

    let next_disabled = current == max;
    let previous_disabled = current == 1;

    let on_click_previous = move |_| on_page_click(current - 1);

    let on_click_next = move |_| on_page_click(current + 1);

    view! {
        cx,
        <div>
            <nav class="pagination" role="navigation" aria-label="pagination">
            <a class="pagination-previous" disabled={previous_disabled} on:click=on_click_previous>"Previous"</a>
            <a class="pagination-next" disabled={next_disabled} on:click=on_click_next>"Next page"
            </a>
            <ul class="pagination-list">
                {lis}
            </ul>
            </nav>
        </div>
    }
}

#[component]
pub fn Ellipsis(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <li>
            <span class="pagination-ellipsis">{ "..." }</span>
        </li>
    }
}

#[component]
pub fn Page<F>(cx: Scope, number: usize, is_current: bool, on_click_number: F) -> impl IntoView
where
    F: Fn(usize) + 'static,
{
    let (aria_current, is_current_s, aria_label) = if is_current {
        (
            "page".to_string(),
            "is-current".to_string(),
            format!("Page {number}"),
        )
    } else {
        (String::new(), String::new(), format!("Goto page {number}"))
    };

    let classes = move || format!("pagination-link {is_current_s}");

    let on_click = move |_| {
        on_click_number(number);
    };

    view! {
        cx,
        <li>
            <a class=classes aria-label={aria_label} aria-current={aria_current}
                on:click=on_click
            >{number}</a>
        </li>
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
