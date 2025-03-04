use crate::utils::*;
use leptos::prelude::*;

#[component]
pub fn Avatar(color: String, color_contrast: f64, theme: Tone) -> impl IntoView {
    let (fill, _) = signal(tone(&color, color_contrast + 0.25, &theme));
    let (color, _) = signal(tone(&color, color_contrast, &theme));

    view! {
        <svg width="40" height="40" viewBox="0 0 40 40" fill="inherit" xmlns="http://www.w3.org/2000/svg" style:color=color.get()>
    <g clip-path="url(#clip0_59820_1088)">
    <circle cx="20" cy="20" r="20" style:fill=fill.clone()/>
    <path d="M20.2312 15.1406L16.8328 25H14.7781L19.0594 13.625H20.3719L20.2312 15.1406ZM23.075 25L19.6687 15.1406L19.5203 13.625H20.8406L25.1375 25H23.075ZM22.9109 20.7812V22.3359H16.7234V20.7812H22.9109Z" style:stroke-width="1px" style:fill={move || color.get()} style:stroke={move || color.get()}/>
    </g>
    <defs>
    <clipPath id="clip0_59820_1088">
    <rect width="40" height="40" fill="white"/>
    </clipPath>
    </defs>
    </svg>
      }
}
