mod app;
pub mod components;
mod utils;

use  app::layout::Layout;

fn main() {
   leptos::mount::mount_to_body(Layout); 
}
