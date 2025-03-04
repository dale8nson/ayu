use crate::components::avatar::*;
use crate::components::button::*;
use crate::components::card::*;
use crate::components::media::*;
use crate::utils::*;
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
      <Stylesheet id="leptos" href="/styles/tailwind.css" />
      <div class="font-roboto text-white text-5xl font-bold grid grid-cols-3 gap-16 p-8 w-full">
      <h1 class="col-span-3 text-5xl tracking-wider font-light">"Buttons"</h1>
        <GradientFillButton>"Gradient"</GradientFillButton>
        <OutlinedButton>"Outlined"</OutlinedButton>
        <ElevatedButton text_col="#ffffff".to_string() theme=Tone::Dark>"Elevated"</ElevatedButton>
        <ElevatedButton text_col="#ee0000".to_string() theme=Tone::Dark>"Toned"</ElevatedButton>
        <TextButton class="border-dashed border-[1px]".to_string()>"Text"</TextButton>
        <div/>
        <h1 class="col-span-3 text-5xl font-light tracking-wider">"Cards"</h1>
        <Card class="col-span-1".to_string() color="#ffffdd".to_string() elevation={1.0} theme={CardTheme::Light}>
        <CardHeader>
        <Avatar color="#ffffdd".to_string() color_contrast={1.3} theme=Tone::Dark />
        <div class="flex flex-col items-start justify-center">
        <h1 class="text-lg font-bold leading-5">"Header"</h1>
        <p class="text-sm font-normal tracking-wide">"Subhead"</p>
        </div>
        </CardHeader>
        <Media style:background-color=tone(&"#ffffdd".to_string(), 1.05, &Tone::Dark) />
        <CardTitle>
        <h3 class="text-base font-bold">"Title"</h3>
        <p class="text-sm font-normal tracking-wide">"Subtitle"</p>
        </CardTitle>
        <CardContent>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor"
        </CardContent>
        </Card>
        <Card class="col-span-1".to_string() color="#ddffdd".to_string() elevation={1.1} theme={CardTheme::Light}>
        <CardHeader>
        <Avatar color="#ddffdd".to_string() color_contrast={1.3} theme=Tone::Dark />
        <div class="flex flex-col items-start justify-center">
        <h1 class="text-lg font-bold leading-4">"Header"</h1>
        <p class="text-sm font-normal tracking-wide">"Subhead"</p>
        </div>
        </CardHeader>
        <Media style:background-color=tone(&"#ddffdd".to_string(), 1.05, &Tone::Dark) />
        <CardTitle>
        <h3 class="text-base font-bold">"Title"</h3>
        <p class="text-sm font-normal tracking-wide">"Subtitle"</p>
        </CardTitle>
        <CardContent>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor"
        </CardContent>
        </Card>
        <Card class="col-span-1".to_string() color="#ffddff".to_string() elevation={1.2} theme={CardTheme::Light}>
        <CardHeader>
        <Avatar color="#ffddff".to_string() color_contrast={1.2} theme=Tone::Dark />
        <div class="flex flex-col items-start justify-center">
        <h1 class="text-lg font-bold leading-4">"Header"</h1>
        <p class="text-sm font-normal tracking-wide">"Subhead"</p>
        </div>
        </CardHeader>
        <Media style:background-color=tone(&"#ffddff".to_string(), 1.3, &Tone::Dark) />
        <CardTitle>
        <h3 class="text-base font-bold">"Title"</h3>
        <p class="text-sm font-normal tracking-wide">"Subtitle"</p>
        </CardTitle>
        <CardContent>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor"
        </CardContent>
        </Card>
        <Card class="col-span-1".to_string() color="#ffffdd".to_string() elevation={1.3} theme={CardTheme::Light}><CardHeader>
        <Avatar color="#ffffdd".to_string() color_contrast={1.4} theme=Tone::Dark />
        <div class="flex flex-col items-start justify-center">
        <h1 class="text-lg font-bold leading-4">"Header"</h1>
        <p class="text-sm font-normal tracking-wide">"Subhead"</p>
        </div>
        </CardHeader>
        <Media style:background-color=tone(&"#ffffdd".to_string(), 1.1, &Tone::Dark) />
        <CardTitle>
        <h3 class="text-base font-bold">"Title"</h3>
        <p class="text-sm font-normal tracking-wide">"Subtitle"</p>
        </CardTitle>
        <CardContent>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor"
        </CardContent>
        </Card>
        <Card class="col-span-1".to_string() color="#ddffdd".to_string() elevation={1.4} theme={CardTheme::Light}><CardHeader>
        <Avatar color="#ddffdd".to_string() color_contrast={1.4} theme=Tone::Dark />
        <div class="flex flex-col items-start justify-center">
        <h1 class="text-lg font-bold leading-4">"Header"</h1>
        <p class="text-sm font-normal tracking-wide">"Subhead"</p>
        </div>
        </CardHeader>
        <Media style:background-color=tone(&"#ddffdd".to_string(), 1.15, &Tone::Dark) />
        <CardTitle>
        <h3 class="text-base font-bold">"Title"</h3>
        <p class="text-sm font-normal tracking-wide">"Subtitle"</p>
        </CardTitle>
        <CardContent>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor"
        </CardContent>
        </Card>
        <Card class="col-span-1".to_string() color="#ffddff".to_string() elevation={1.5} theme={CardTheme::Light}>
        <CardHeader>
        <Avatar color="#ffddff".to_string() color_contrast={1.2} theme=Tone::Dark />
        <div class="flex flex-col items-start justify-center">
        <h1 class="text-lg font-bold leading-4">"Header"</h1>
        <p class="text-sm font-normal tracking-wide">"Subhead"</p>
        </div>
        </CardHeader>
        <Media style:background-color=tone(&"#ffddff".to_string(), 1.3, &Tone::Dark) />
        <CardTitle>
        <h3 class="text-base font-bold">"Title"</h3>
        <p class="text-sm font-normal tracking-wide">"Subtitle"</p>
        </CardTitle>
        <CardContent>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor"
        </CardContent>
        </Card>
        <Card class="col-span-1".to_string() color="#ffffdd".to_string() elevation={1.6} theme={CardTheme::Light}>
        <CardHeader>
        <Avatar color="#ffffdd".to_string() color_contrast={1.3} theme=Tone::Dark />
        <div class="flex flex-col items-start justify-center">
        <h1 class="text-lg font-bold leading-4">"Header"</h1>
        <p class="text-sm font-normal tracking-wide">"Subhead"</p>
        </div>
        </CardHeader>
        <Media style:background-color=tone(&"#ffffdd".to_string(), 1.3, &Tone::Dark) />
        <CardTitle>
        <h3 class="text-base font-bold">"Title"</h3>
        <p class="text-sm font-normal tracking-wide">"Subtitle"</p>
        </CardTitle>
        <CardContent>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor"
        </CardContent>
        </Card>
        <Card class="col-span-1".to_string() color="#ddffdd".to_string() elevation={1.7} theme={CardTheme::Light}>
        <CardHeader>
        <Avatar color="#ddffdd".to_string() color_contrast={1.3} theme=Tone::Dark />
        <div class="flex flex-col items-start justify-center">
        <h1 class="text-lg font-bold leading-4">"Header"</h1>
        <p class="text-sm font-normal tracking-wide">"Subhead"</p>
        </div>
        </CardHeader>
        <Media style:background-color=tone(&"#ddffdd".to_string(), 1.4, &Tone::Dark) />
        <CardTitle>
        <h3 class="text-base font-bold">"Title"</h3>
        <p class="text-sm font-normal tracking-wide">"Subtitle"</p>
        </CardTitle>
        <CardContent>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor"
        </CardContent>
        </Card>
        <Card class="col-span-1".to_string() color="#ffddff".to_string() elevation={1.8} theme={CardTheme::Light}>
        <CardHeader>
        <Avatar color="#ffddff".to_string() color_contrast={1.2} theme=Tone::Dark />
        <div class="flex flex-col items-start justify-center">
        <h1 class="text-lg font-bold leading-4">"Header"</h1>
        <p class="text-sm font-normal tracking-wide">"Subhead"</p>
        </div>
        </CardHeader>
        <Media style:background-color=tone(&"#ffddff".to_string(), 1.4, &Tone::Dark) />
        <CardTitle>
        <h3 class="text-base font-bold">"Title"</h3>
        <p class="text-sm font-normal tracking-wide">"Subtitle"</p>
        </CardTitle>
        <CardContent>
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor"
        </CardContent>
        </Card>
      </div>
    }
}
