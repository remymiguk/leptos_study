use leptos::*;

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

    let on_click_previous = move |_| {
        if !previous_disabled {
            on_page_click(current - 1)
        }
    };

    let on_click_next = move |_| {
        if !next_disabled {
            on_page_click(current + 1)
        }
    };

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
