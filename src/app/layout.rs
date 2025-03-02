use crate::components::button::{FilledButton, OutlinedButton, TextButton, ElevatedButton};
use crate::utils::SecondaryColorType;
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Layout() -> impl IntoView {

    view! {
      <Stylesheet id="leptos" href="/styles/tailwind.css" />
      <div class="font-roboto text-white text-5xl font-bold grid grid-cols-5 gap-16 m-8 w-full">
      <h1 class="col-span-5 text-2xl">Buttons</h1>
        <FilledButton>"Filled"</FilledButton>
        <OutlinedButton>"Outlined"</OutlinedButton>
        <ElevatedButton text_col="#ffffff".to_string() theme=SecondaryColorType::Dark>"Elevated"</ElevatedButton>
        <ElevatedButton text_col="#ee0000".to_string() theme=SecondaryColorType::Dark>"Toned"</ElevatedButton>
        <TextButton>"Text"</TextButton>
      </div>
    }
}
