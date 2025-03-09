use crate::utils::*;
use leptos::prelude::*;

#[component]
pub fn Button(children: Children, #[prop(optional)] class: String) -> impl IntoView {
    view! {
        <button class=format!("flex flex-row justify-center items-center w-auto h-[4rem] text-base text-center font-normal py-2 px-2 rounded-[2rem] cursor-pointer hover:cursor-pointer aspect-video {}", class)>
            {children()}
        </button>
    }
}

#[component]
pub fn GradientFillButton(
    children: Children,
    #[prop(optional)] class: String,
    #[prop(optional)] fill: Option<String>,
) -> impl IntoView {
    let (btn, set_btn) = signal(false);
    let (hover, set_hover) = signal(false);
    let fill = fill.unwrap_or("#2b7fff".to_string());
    let fill = fill.clone();

    let (gradient, set_gradient) = signal(format!(
        "linear-gradient(to bottom, {}, {})",
        fill,
        secondary_col(&fill[..], &Tone::Dark)
    ));

    let secondary_fill_color = secondary_col(&fill, &Tone::Dark);
    let secondary_fill_color_clone = secondary_fill_color.clone();

    let fill_clone = fill.clone();
    let fill_color = Memo::new(move |_| {
        if btn.get() {
            fill.to_string()
        } else if hover.get() {
            secondary_fill_color.clone()
        } else {
            fill.to_string()
        }
    });

    let secondary_fill_color = Memo::new(move |_| {
        if btn.get() {
            secondary_fill_color_clone.to_string()
        } else if hover.get() {
            fill_clone.to_string()
        } else {
            secondary_fill_color_clone.to_string()
        }
    });

    Effect::new(move |_| {
        set_gradient.set(format!(
            "linear-gradient(to bottom, {}, {})",
            fill_color.get(),
            secondary_fill_color.get()
        ));
    });

    let on_pointer_enter = move |_| set_hover.set(true);
    let on_pointer_leave = move |_| set_hover.set(false);
    let on_mousedown = move |_| set_btn.set(true);
    let on_mouseup = move |_| set_btn.set(false);

    view! {
        <Button on:mousedown = on_mousedown on:mouseup = on_mouseup
        on:mouseenter = on_pointer_enter on:mouseleave = on_pointer_leave
        class:hover:bg-linear-to-t = move || { !btn.get() } class=format!("bg-linear-to-b from-blue-500 to-blue-700 hover: hover:bg-linear-to-t inset-sm hover:shadow-white text-white {}", class)
        style:background-image=move || gradient.get()>
        {children()}
        </Button>
    }
}

#[component]
pub fn FilledButton(
    children: Children,
    fill: String,
    contrast: f64,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or("".to_string());

    let (btn, set_btn) = signal(false);

    let (r, g, b) = css_to_rgb(&fill);
    let (l, a, b) = srgb_to_oklab(r, g, b);
    // let l = l_contrast(l, 0.0, &Tone::Light);
    let bl = l.clone();
    let color: Memo<String> = Memo::new(move |_| {
        let l = if l < 128.0 {
            if btn.get() {
                l
            } else {
                l_contrast(l, contrast, &Tone::Light)
                
            }
        } else {
            if btn.get() {
                l_contrast(l, contrast, &Tone::Light)
            } else {
                l
            }
        };

        let (cr, cg, cb) = oklab_to_srgb(l, a, b);
        format!("#{:02x}{:02x}{:02x}", cr, cg, cb)
    });

    let fill: Memo<String> = Memo::new(move |_| {
        let bl = if bl < 128.0 {
            if btn.get() {
                l_contrast(bl, contrast, &Tone::Light)
                
            } else {
                bl
            }
        } else {
            if btn.get() {
                bl
            } else {
                l_contrast(bl, contrast, &Tone::Light)
            }
        };

        let (br, bg, bb) = oklab_to_srgb(bl, a, b);
        format!("#{:02x}{:02x}{:02x}", br, bg, bb)
    });


    let on_mousedown = move |_| set_btn.set(true);
    let on_mouseup = move |_| set_btn.set(false);

    view! {
        <Button on:mouseup = on_mouseup on:mousedown = on_mousedown class=format!("px-4 py-2 aspect-video {}", class) style:background-color=fill style:color=color>
        {children()}
        </Button>
    }
}

#[component]
pub fn OutlinedButton(#[prop(optional)] color: Option<String>, contrast: f64, children: Children, #[prop(optional)] class: String) -> impl IntoView {
    let (color, _) = signal(color.unwrap_or("#aaaaaa".to_string()));
    let (btn, set_btn) = signal(false);
    let (r, g, b) = css_to_rgb(&color.get());
    let (hover_color, _) = signal(tone(&color.get(), contrast, if srgb_to_oklab(r, g, b).0 > 128.0 { &Tone::Light } else { &Tone::Dark }));
    let (border_color, set_border_color) = signal(color.get());

    view! {
        <Button on:mouseenter=move |_| set_border_color.set(hover_color.get())
        on:mouseleave=move |_| set_border_color.set(color.get()) on:mousedown = move |_| set_border_color.set(color.get()) on:mouseup = move |_| set_border_color.set(hover_color.get()) on:click = move |_| {} class:hover:border-white = move || !btn.get() class=format!("border-2 border-solid hover:inset-shadow-xs {}", class)
        style:border-color=move || border_color.get() style:color=move || border_color.get()
        >
        {children()}
        </Button>
    }
}

#[component]
pub fn TextButton(children: Children, #[prop(optional)] class: Option<String>) -> impl IntoView {
    let (btn, set_btn) = signal(false);
    let class = class.unwrap_or("".to_string());
    view! {
        <button on:mousedown = move |_| set_btn.set(true) on:mouseup = move |_| set_btn.set(false) class:hover:text-white = move || !btn.get() class=format!("text-center w-fit p-4 h-[4rem] text-base font-normal text-[#aaa] hover:border-white hover:text-white cursor-pointer hover:cursor-pointer {}", class)>
        {children()}
        </button>
    }
}

#[component]
pub fn ElevatedButton(
    children: Children,
    #[prop(optional)] class: String,
    #[prop(optional)] text_col: Option<String>,
    #[prop(optional)] theme: Option<Tone>,
) -> impl IntoView {
    let (btn, set_btn) = signal(false);
    let (hover, set_hover) = signal(false);

    let text_col = text_col.unwrap_or("#ffffff".to_string());
    let (sig_color, set_sig_color) = signal(text_col.clone());
    let (sig_bg_color, set_sig_bg_color) = signal(String::from("#000000"));

    let (r, g, b) = css_to_rgb(&text_col[..]);
    let (l, _, _) = srgb_to_oklab(r, g, b);
    let theme = theme.unwrap_or(if l < 0.5 { Tone::Light } else { Tone::Dark });

    let theme_clone = theme.clone();
    let text_col_clone = text_col.clone();

    let color = Memo::new(move |_| {
        if btn.get() {
            text_col.clone()
        } else if hover.get() {
            secondary_col(&text_col, &theme)
        } else {
            text_col.clone()
        }
    });

    let bg_color = Memo::new(move |_| {
        if btn.get() {
            secondary_col(&text_col_clone, &theme_clone)
        } else if hover.get() {
            String::from(&text_col_clone)
        } else {
            secondary_col(&text_col_clone, &theme_clone)
        }
    });

    Effect::new(move |_| {
        set_sig_color.set(color.get());
        set_sig_bg_color.set(bg_color.get());
    });

    let on_pointer_enter = move |_| set_hover.set(true);
    let on_pointer_leave = move |_| set_hover.set(false);
    let on_mousedown = move |_| set_btn.set(true);
    let on_mouseup = move |_| set_btn.set(false);

    view! {
        <Button on:mouseenter = on_pointer_enter on:mouseleave = on_pointer_leave on:mousedown = on_mousedown on:mouseup = on_mouseup class:hover:border-white = move || !btn.get() class:hover:shadow-white =move || !btn.get() class:shadow-aaa = move || !btn.get() class:shadow-md =move || !btn.get() class:shadow-none =move || btn.get()
        style:color=move || sig_color.get()
        style:background-color=move || sig_bg_color.get()
         class=format!("border-2 border-solid border-[#aaa]  hover:border-white hover:inset-shadow-xs shadow-md hover:shadow-white {}", class)>
        {children()}
        </Button>
    }
}

#[component]
pub fn ExportButton(#[prop(optional)] color: Option<String>, #[prop(optional)] contrast:Option<f64>, #[prop(optional)] class: Option<String>) -> impl IntoView {
    let color = color.unwrap_or("#aaa".to_string()); 
    let contrast = contrast.unwrap_or(1.0);
    let class = class.unwrap_or("".to_string());

    // let (btn, set_btn) = signal(false);
    let (hover, set_hover) = signal(false);

    view! {
        <Button on:pointerenter=move |_| set_hover.set(true) on:pointerleave=move |_| set_hover.set(false) class=format!("flex flex-row justify-center items-center py-2 w-[4rem] h-[4rem] rounded-full m-auto {}", class).to_string() style:background-color=move || if hover.get() { tone(&color, contrast + 0.1, &Tone::Dark) } else { tone(&color.clone(), contrast, &Tone::Dark) }>
        <svg width="4" height="16" viewBox="0 0 4 16" fill="current" xmlns="http://www.w3.org/2000/svg">
        <path d="M2 16C1.45 16 0.979167 15.8042 0.5875 15.4125C0.195833 15.0208 0 14.55 0 14C0 13.45 0.195833 12.9792 0.5875 12.5875C0.979167 12.1958 1.45 12 2 12C2.55 12 3.02083 12.1958 3.4125 12.5875C3.80417 12.9792 4 13.45 4 14C4 14.55 3.80417 15.0208 3.4125 15.4125C3.02083 15.8042 2.55 16 2 16ZM2 10C1.45 10 0.979167 9.80417 0.5875 9.4125C0.195833 9.02083 0 8.55 0 8C0 7.45 0.195833 6.97917 0.5875 6.5875C0.979167 6.19583 1.45 6 2 6C2.55 6 3.02083 6.19583 3.4125 6.5875C3.80417 6.97917 4 7.45 4 8C4 8.55 3.80417 9.02083 3.4125 9.4125C3.02083 9.80417 2.55 10 2 10ZM2 4C1.45 4 0.979167 3.80417 0.5875 3.4125C0.195833 3.02083 0 2.55 0 2C0 1.45 0.195833 0.979167 0.5875 0.5875C0.979167 0.195833 1.45 0 2 0C2.55 0 3.02083 0.195833 3.4125 0.5875C3.80417 0.979167 4 1.45 4 2C4 2.55 3.80417 3.02083 3.4125 3.4125C3.02083 3.80417 2.55 4 2 4Z" fill="inherit"/>
        </svg>
        </Button>
    }
}
