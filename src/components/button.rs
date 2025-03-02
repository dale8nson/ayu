use crate::utils::*;
use leptos::prelude::*;

#[component]
pub fn Button(children: Children, #[prop(optional)] class: String) -> impl IntoView {
    view! {
        <button class=format!("w-3/4 h-[4rem] text-base font-normal py-2 px-2 rounded-[2rem] {}", class)>
            {children()}
        </button>
    }
}

#[component]
pub fn FilledButton(children: Children, #[prop(optional)] class: String, #[prop(optional)] fill: Option<String>) -> impl IntoView {
    let (btn, set_btn) = signal(false);
    let (hover, set_hover) = signal(false);
    let fill = fill.unwrap_or("#2b7fff".to_string());
    let fill = fill.clone();

    let (gradient, set_gradient) = signal(format!("linear-gradient(to bottom, {}, {})", fill, secondary_col(&fill[..], &SecondaryColorType::Dark)));
    
    let secondary_fill_color  = secondary_col(&fill, &SecondaryColorType::Dark);
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
        set_gradient.set(format!("linear-gradient(to bottom, {}, {})", fill_color.get(), secondary_fill_color.get()));
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
pub fn OutlinedButton(children: Children, #[prop(optional)] class: String) -> impl IntoView {
    let (btn, set_btn) = signal(false);

    view! {
        <Button on:mousedown = move |_| *set_btn.write() = true on:mouseup = move |_| *set_btn.write() = false on:click = move |_| {} class:hover:border-white = move || !btn.get() class:hover:text-white = move || !btn.get() class=format!("border-2 border-solid border-[#aaa] text-[#aaa] hover:border-white hover:text-white hover:inset-shadow-xs inset-shadow-white {}", class)>
        {children()}
        </Button>
    }
}

#[component]
pub fn TextButton(children: Children, #[prop(optional)] class: String) -> impl IntoView {
    let (btn, set_btn) = signal(false);

    view! {
        <button on:mousedown = move |_| set_btn.set(true) on:mouseup = move |_| set_btn.set(false) class:hover:text-white = move || !btn.get() class=format!(" text-base font-normal text-[#aaa] hover:border-white hover:text-white {}", class)>
        {children()}
        </button>
    }
}

#[component]
pub fn ElevatedButton(
    children: Children,
    #[prop(optional)] class: String,
    #[prop(optional)] text_col: Option<String>,
    #[prop(optional)] theme: Option<SecondaryColorType>,
) -> impl IntoView {
    let (btn, set_btn) = signal(false);
    let (hover, set_hover) = signal(false);

    let text_col = text_col.unwrap_or("#ffffff".to_string());
    let (sig_color, set_sig_color) = signal(text_col.clone());
    let (sig_bg_color, set_sig_bg_color) = signal(String::from("#000000"));

    let (r, g, b) = css_to_rgb(&text_col[..]);
    let (l, _, _) = srgb_to_oklab(r, g, b);
    let theme = theme.unwrap_or(if l < 0.5 {
        SecondaryColorType::Light
    } else {
        SecondaryColorType::Dark
    });

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
    // let color = if btn.get() {
    //         text_col.clone()
    //     } else if hover.get() {
    //         secondary_col(&text_col, &theme)
    //     } else {
    //         text_col.clone()
    //     };

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
