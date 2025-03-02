use leptos::prelude::*;
use crate::utils::*;

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
    CardTheme::Light => tone(&color, elevation, &SecondaryColorType::Dark),
    CardTheme::Dark => tone(&color, elevation, &SecondaryColorType::Light),
  });

  let (bg2, _) = signal(match theme {
    CardTheme::Light => tone(&color, elevation + (0.1 * elevation), &SecondaryColorType::Dark),
    CardTheme::Dark => tone(&color, elevation - (0.1 * elevation), &SecondaryColorType::Light),
  });

  view! {
    <div class=format!("w-[22.5rem] h-[30rem] rounded-2xl {}", class) style:background-color=move || bg.get() style:box-shadow={move || format!("{}px {}px {}px {}, -{}px {}px {}px {}", elevation * 3.0, elevation * 4.0, elevation * 0.25, bg2.get(), elevation * 3.0, elevation * 4.0, elevation * 0.25, bg2.get())}>
    {children()}
    </div>
  }
}