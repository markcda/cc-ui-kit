use dioxus::prelude::*;

use crate::hvalues::HValue;
use crate::components::coloring_rules::CCDioxusV04ColoringRules;

/// Status text component.
///
/// Code name for coloring rules: `status_text` (see `crate::components::coloring_rules`).
pub fn StatusText<'a>(cx: Scope<'a>, notification_text: HValue<String>) -> Element<'a> {
  let coloring_rules = if
    let Some(coloring) = use_shared_state::<CCDioxusV04ColoringRules>(cx) &&
    let Some(status_text_rules) = coloring.read().additional_styles.get("status_text")
  { status_text_rules.to_owned() } else { "decoration-[#607a79] text-[#607a79] underline italic".into() };

  cx.render({
    let text = if let Some(notification_text) = notification_text.read_ref() { notification_text.to_owned() } else { String::new() };
    let hidden = if text.is_empty() { "hidden" } else { "" };

    rsx! {
      div {
        class: "{coloring_rules} {hidden}",
        "{text}"
      }
    }
  })
}