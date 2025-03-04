use leptos::prelude::*;
use crate::utils::*;
use crate::components::button::*;

pub enum CardTheme {
  Light,
  Dark,
}

#[component]
pub fn Card(children: Children, #[prop(optional)] class: Option<String>, #[prop(optional)] color: Option<String>, #[prop(optional)] elevation: Option<f64>, #[prop(optional)] theme: Option<CardTheme> ) -> impl IntoView {
  let color = color.unwrap_or("#ffffff".to_string());
  let class = class.unwrap_or("".to_string());
  let elevation = elevation.unwrap_or(1.0);
  let theme = theme.unwrap_or(CardTheme::Light);

  let (bg, _) = signal(match theme {
    CardTheme::Light => tone(&color, elevation, &Tone::Dark),
    CardTheme::Dark => tone(&color, elevation, &Tone::Light),
  });

  let (bg2, _) = signal(match theme {
    CardTheme::Light => tone(&color, elevation + (0.8 * elevation), &Tone::Dark),
    CardTheme::Dark => tone(&color, elevation - (0.8 * elevation), &Tone::Light),
  });

  view! {
    <div class={move || format!("flex flex-col items-start gap-y-2 w-10/12 h-auto rounded-2xl pb-8 mx-auto  {}", class)} style:background-color=move || bg.get() style:color=move || bg2.get() style:box-shadow={move || format!("{}px {}px {}px {}, -{}px {}px {}px {}", elevation * 3.0, elevation * 4.0, elevation * 3.0, bg2.get(), elevation * 3.0, elevation * 4.0, elevation * 3.0, bg2.get())}
    style:fill = {move || bg2.get()} style:stroke = {move || bg.get()}
    >
    <div>{children()}</div>
    <div class="flex flex-row justify-end items-center gap-x-4 w-full px-4">
    <FilledButton fill=bg2.get()>"CTA"</FilledButton>
    </div>
    </div>
  }
}

#[component]
pub fn CardHeader(children: Children, #[prop(optional)] class:Option<String> ) -> impl IntoView {
let class = class.unwrap_or("".to_string());
  view! {
    <div class=format!("flex flex-row justify-between items-center w-full h-[4rem] px-4 m-0 {} ", class) >
    <div class="flex flex-row gap-x-4 items-center w-full h-full">
    {children()}
    </div>
    <ExportButton />
    </div>
  }
}

#[component]
pub fn CardTitle(children: Children, #[prop(optional)] class:Option<String> ) -> impl IntoView {
let class = class.unwrap_or("".to_string());
  view! {
    <div class=format!("flex flex-col items-start justify-center px-4 pt-4 {}", class)>
    {children()}
    </div>
  }
}

#[component]
pub fn CardContent(children: Children, #[prop(optional)] class:Option<String> ) -> impl IntoView {
  let class = class.unwrap_or("".to_string());
  view! {
    <div class=format!("flex flex-col items-start justify-center px-4 py-4 text-sm font-normal leading-4 {}", class)>
    {children()}
    </div>
  }
}
