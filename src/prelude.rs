pub use leptos::prelude::*;

pub use console_error_panic_hook;
pub use console_log;
pub use log;

pub use lucide_leptos;
pub use thaw::*;

pub fn setup_app(#[allow(unused_variables)] log_level: log::Level, children: Children) {
  console_error_panic_hook::set_once();
  #[cfg(debug_assertions)]
  console_log::init_with_level(log::Level::Debug).unwrap();
  #[cfg(not(debug_assertions))]
  console_log::init_with_level(log_level).unwrap();
  leptos::mount::mount_to_body(move || {
    view! { <UIApp children /> }
  })
}

#[component]
pub fn UIApp(children: Children) -> impl IntoView {
  use crate::utils::{cn, dark_theme, light_theme};

  let preferred_dark = leptos_use::use_preferred_dark();
  let tw_dark_class = RwSignal::new(if preferred_dark.get() {
    Some("dark".to_string())
  } else {
    None
  });
  let theme = RwSignal::new({
    if preferred_dark.get() {
      dark_theme()
    } else {
      light_theme()
    }
  });
  Effect::new(move |_| {
    theme.set(if preferred_dark.get() {
      dark_theme()
    } else {
      light_theme()
    })
  });

  view! {
    <crate::style_config::ConfigProvider
      theme
      style="min-height: 100%; min-width: 100%; overflow-x: auto;"
      class=cn("", tw_dark_class.get())
    >
      <div style="display: flex; flex-direction: column;">{children()}</div>
    </crate::style_config::ConfigProvider>
  }
}
