use crate::{
    models::product::{Product, ProductModel},
    states::{app_state::StateGetter, form_object::*, object_model::ObjectModelBuilder},
};
use leptos::*;
use leptos_router::use_params_map;
use log::info;
use uuid::Uuid;

#[component]
pub fn ProductForm(cx: Scope) -> impl IntoView {
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

    // let t = move || product.read();

    // let child = move || match product.read() {
    //     Some(product) => match product {
    //         Some(product) => {
    //             info!("#3");
    //             view! { cx, <LoadedProductForm product/> }.into_view(cx)
    //         }
    //         None => view! { cx, <div class="item-view">"Error loading this product."</div> }
    //             .into_view(cx),
    //     },
    //     None => {
    //         info!("#2");
    //         view! { cx, "Loading..." }.into_view(cx)
    //     }
    // };

    view! {
        cx,
        <HoldOn
            read={move || product.read()}
            fallback={move ||view! { cx, "Loading..." }.into_view(cx)}
            error={move ||view! { cx, <div class="item-view">"Error loading this product."</div> }.into_view(cx)}
            child={move |product| view! { cx, <LoadedProductForm product/> }.into_view(cx)}
        />
    }
}

#[component]
pub fn HoldOn<T, R, F, E, V, C>(
    cx: Scope,
    read: R,
    fallback: F,
    error: E,
    child: C,
) -> impl IntoView
where
    R: Fn() -> Option<Option<T>> + 'static,
    F: Fn() -> V + 'static,
    E: Fn() -> V + 'static,
    C: Fn(T) -> V + 'static,
    V: IntoView,
{
    move || match read() {
        Some(result) => match result {
            Some(payload) => child(payload).into_view(cx),
            None => error().into_view(cx),
        },
        None => fallback().into_view(cx),
    }
}

#[component]
pub fn ProductFormTest(cx: Scope) -> impl IntoView {
    info!("#1");

    let params = use_params_map(cx);
    let model = use_context::<StateGetter<ProductModel>>(cx).unwrap().0();

    let product = create_local_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        move |id| {
            let model = model.clone();
            async move {
                // fake API delay
                // let _ = wasm_timer::Delay::new(Duration::from_secs(1)).await;

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

    // let product = create_local_resource(
    //     cx,
    //     move || {},
    //     move |_| async move {
    //         // fake API delay
    //         std::thread::sleep(std::time::Duration::from_millis(1250));
    //         Some(Product {
    //             id: Uuid::default(),
    //             description: String::from("product"),
    //             category: Uuid::default(),
    //             price: dec!(1.0),
    //             created_at: Utc::now().naive_utc(),
    //         })
    //     },
    // );

    let child = move || match product.read() {
        Some(product) => match product {
            Some(product) => {
                info!("#3");
                view! { cx, <LoadedProductForm product/> }.into_view(cx)
            }
            None => view! { cx, <div class="item-view">"Error loading this product."</div> }
                .into_view(cx),
        },
        None => {
            info!("#2");
            view! { cx, "Loading..." }.into_view(cx)
        }
    };

    // view! {
    //     cx,
    //     { child }
    // }

    view! {
        cx,
        <Suspense fallback=|| { info!("#2"); view! { cx, "Loading..." } }>
            {move || product.read().map(|product| match product {
                None => view! { cx, <div class="item-view">"Error loading this product."</div> }.into_view(cx),
                Some(product) => {
                    info!("#3");
                    view! { cx, <LoadedProductForm product/> }.into_view(cx)
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn LoadedProductFormTest(cx: Scope, product: Product) -> impl IntoView {
    info!("#4");

    let (read_signal, _) = create_signal(cx, product);

    let read_resource = create_local_resource(cx, read_signal, |arg| async move { arg });

    // If comment this line then it fixes infinite loop
    let _ = create_memo(cx, move |_| read_resource.read());

    view! { cx,
        <div>
            <input
                class="button is-danger"
                on:click=move |_| navigator_back()
                type="button"
                value="Cancel"/>
        </div>
    }
}

#[component]
pub fn LoadedProductForm(cx: Scope, product: Product) -> impl IntoView {
    info!("***************");

    let model = ObjectModelBuilder::new(cx, product).build();
    let fo = FormObject::new(model);

    view! { cx,
        <div>
            <InputBind fo=&fo input_type="text" literal="Id" field_name="id" placeholder="Id"/>
            <InputBind fo=&fo input_type="text" literal="Description" field_name="description" placeholder="Description"/>
            <InputBind fo=&fo input_type="text" literal="Price" field_name="price" placeholder="Price"/>
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
