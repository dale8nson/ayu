use leptos::prelude::*;

#[component]
pub fn Media(#[prop(optional)] class:String) -> impl IntoView {
    view! {
        <img src="/assets/placeholder-image Background Removed.png" class=format!("w-full h-1/2 object-cover bg-transparent {}", class) />

      }
}
