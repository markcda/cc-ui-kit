use crate::prelude::*;

/// Компонент текста со статусом операции.
///
/// Если текста нет, то компонент не отображается.
///
/// Компонент может использоваться, когда нужно произвести обработку или отправку каких-либо данных с последующим
/// выводом статуса операции.
///
/// Кодовое имя для `CCUiKitRules`: `status_text` (см. `crate::components::coloring_rules::CCUiKitRules`).
pub fn StatusText<'a>(cx: Scope<'a>, notification_text: HValue<String>) -> Element<'a> {
  let coloring_rules = if
    let Some(coloring) = use_shared_state::<CCUiKitRules>(cx) &&
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