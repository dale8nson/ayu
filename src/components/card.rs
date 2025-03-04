use crate::components::button::*;
use crate::utils::*;
use leptos::prelude::*;

pub enum CardTheme {
    Light,
    Dark,
}

#[component]
pub fn Card(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] elevation: Option<f64>,
    #[prop(optional)] theme: Option<CardTheme>,
) -> impl IntoView {
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
      <div class={move || format!("relative flex flex-col items-start overflow-clip  gap-y-2 w-full h-auto rounded-2xl pb-8 mx-auto  {}", class)} style:background-color=move || bg.get() style:color=move || bg2.get() style:box-shadow={move || format!("{}px {}px {}px {}, -{}px {}px {}px {}", elevation * 3.0, elevation * 4.0, elevation * 3.0, bg2.get(), elevation * 3.0, elevation * 4.0, elevation * 3.0, bg2.get())}
      style:fill = {move || bg2.get()} style:stroke = {move || bg.get()}
      >
      <div>{children()}</div>
      <div class="flex flex-row justify-end items-center gap-x-4 w-full px-4">
      <OutlinedButton color=bg2.get() class="w-auto".to_string() contrast=2.0 >"Close"</OutlinedButton>
      <FilledButton class="w-auto".to_string() fill=bg2.get() contrast=3.0>"CTA"</FilledButton>
      </div>
      </div>
    }
}

#[component]
pub fn CardHeader(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] contrast: Option<f64>,
) -> impl IntoView {
    let class = class.unwrap_or("".to_string());
    let color = color.unwrap_or("#aaaaaa".to_string());
    let contrast = contrast.unwrap_or(1.0);
    view! {
      <div class=format!("flex flex-row relative z-20 justify-between items-center w-full h-[5.5rem] px-4 m-0 {} ", class) >
      <div class="flex flex-row gap-x-4 items-center w-full h-full">
      {children()}
      </div>
      <CardMenu color=color contrast=contrast class="relative -z-10".to_string()>
      <li><a>"Menu item 1"</a></li>
      <li>"Menu item 2" </li>
      <li>"Menu item 3" </li>
      <li>"Menu item 4" </li>
      <li>"Menu item 5" </li>
      </CardMenu>
      </div>
    }
}

#[component]
pub fn CardTitle(children: Children, #[prop(optional)] class: Option<String>) -> impl IntoView {
    let class = class.unwrap_or("".to_string());
    view! {
      <div class=format!("flex flex-col items-start justify-center px-4 pt-4 {}", class)>
      {children()}
      </div>
    }
}

#[component]
pub fn CardContent(children: Children, #[prop(optional)] class: Option<String>) -> impl IntoView {
    let class = class.unwrap_or("".to_string());
    view! {
      <div class=format!("flex flex-col items-start justify-center px-4 py-4 text-sm font-normal leading-4 {}", class)>
      {children()}
      </div>
    }
}

#[component]
pub fn CardMenu(
    children: Children,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] contrast: Option<f64>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let color = color.unwrap_or("#aaaaaa".to_string());
    let contrast = contrast.unwrap_or(1.0);
    let class = class.unwrap_or("".to_string());
    let (open, set_open) = signal(false);

    view! {
      <div class=format!("relative flex flex-col items-end justify-around gap-y-2 w-full h-full translate-y-3 {}", class)>
        <ExportButton class="relative z-[10]".to_string() on:click=move |_| set_open.set(!open.get()) color=color.clone() contrast=contrast />
        <div class="relative -z-50 p-4 w-full h-fit rounded-lg transition-layout duration-50 shadow" style:background-color={tone(&color.clone(), contrast, &Tone::Dark)} style:visibility=move || if open.get() {"visible"} else { "hidden" }
        style:transform=move || if open.get() { "translateY(0)".to_string()} else { "translateY(-5rem)".to_string()}>
        <ul class="text-xl relative z-[-5]">
        {children()}
        </ul>
        </div>
      </div>

    }
}
