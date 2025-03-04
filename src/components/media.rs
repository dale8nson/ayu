use leptos::prelude::*;

#[component]
pub fn Media(#[prop(optional)] class:String) -> impl IntoView {
    view! {
        <img width="360" height="188" src="/assets/placeholder-image Background Removed.png" class=format!("min-w-full h-1/2 object-fit bg-transparent {}", class) />

      }
}
