use leptos::prelude::*;
use thaw::*;
use thaw_utils::{mount_dynamic_style, mount_style};

#[component]
pub fn ConfigProvider(
  #[prop(optional, into)] style: Option<String>,
  #[prop(optional, into)] class: Option<String>,
  /// Sets the theme used in a scope.
  #[prop(optional, into)]
  theme: Option<RwSignal<Theme>>,
  /// Theme id
  #[prop(optional, into)]
  theme_id: Option<String>,
  /// Sets the direction of text & generated styles.
  #[prop(optional, into)]
  dir: Option<RwSignal<ConfigDirection>>,
  children: Children,
) -> impl IntoView {
  mount_style(
    "config-provider",
    ".thaw-config-provider {
    font-family: var(--fontFamilyBase);
    font-size: var(--fontSizeBase300);
    line-height: var(--lineHeightBase300);
    font-weight: var(--fontWeightRegular);
    background-color: var(--colorNeutralBackground1);
    color: var(--colorNeutralForeground1);
}",
  );

  let theme = theme.unwrap_or_else(|| RwSignal::new(Theme::light()));
  let theme_id = theme_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
  let id = StoredValue::new(theme_id);

  mount_dynamic_style(id.get_value(), move || {
    let mut css_vars = String::new();
    theme.with(|theme| {
      theme.common.write_css_vars(&mut css_vars);
      theme.color.write_css_vars(&mut css_vars);
    });
    format!(
      ".thaw-config-provider[data-thaw-id=\"{}\"]{{{css_vars}}}",
      id.get_value()
    )
  });

  Owner::on_cleanup(move || {
    if let Ok(Some(style)) =
      document().query_selector(&format!("head style[data-thaw-id=\"{}\"]", id.get_value()))
    {
      style.remove();
    }
  });

  let config_injection = ConfigInjection { theme, dir, id };

  view! {
    <leptos::context::Provider value=config_injection>
      <div
        style=style
        class=crate::utils::cn("thaw-config-provider", class)
        data-thaw-id=id.get_value()
        dir=move || dir.map(move |dir| dir.get().as_str())
      >
        {children()}
      </div>
    </leptos::context::Provider>
  }
}

#[derive(Clone)]
pub struct ConfigInjection {
  pub theme: RwSignal<Theme>,
  pub dir: Option<RwSignal<ConfigDirection>>,
  pub id: StoredValue<String>,
}

impl ConfigInjection {
  pub fn id(&self) -> String {
    self.id.get_value()
  }

  pub fn expect_context() -> Self {
    expect_context()
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ConfigDirection {
  Ltr,
  Rtl,
  #[default]
  Auto,
}

impl ConfigDirection {
  pub fn as_str(&self) -> &'static str {
    match self {
      ConfigDirection::Ltr => "ltr",
      ConfigDirection::Rtl => "rtl",
      ConfigDirection::Auto => "auto",
    }
  }
}
