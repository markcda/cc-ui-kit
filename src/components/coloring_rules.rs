//! Модуль для простой стилизации компонентов интерфейса CC UI Kit.

use std::collections::HashMap;

/// Правила оформления интерфейса. Интегрируются с Dioxus и Tailwind CSS.
#[derive(Default, Clone)]
pub struct CCUiKitRules {
  /// Основной цвет интерфейса (текст, границы, акцент и пр.).
  pub primary_color: String,
  /// Дополнительный цвет (фон).
  pub secondary_color: String,
  /// Основной цвет интерфейса (текст, границы, акцент и пр.) при наведении. Если нет, используется `primary_color`.
  pub primary_color_hovered: String,
  /// Дополнительный цвет (фон) при наведении. Если нет, используется `secondary_color`.
  pub secondary_color_hovered: String,
  /// Основной цвет интерфейса (текст, границы, акцент и пр.) при нажатии. Если нет, используется `primary_color`.
  pub primary_color_selected: String,
  /// Дополнительный цвет (фон) при нажатии. Если нет, используется `secondary_color`.
  pub secondary_color_selected: String,
  /// Дополнительные стили.
  ///
  /// Это - набор классов Tailwind CSS, которые применяются к названным компонентам интерфейса. Названия компонентов
  /// необходимо уточнять в документации к компонентам.
  ///
  /// Пример использования:
  ///
  /// ```rust
  /// use cc_ui_kit::components::coloring_rules::CCUiKitRules;
  ///
  /// let mut coloring_rules: CCUiKitRules = Default::default();
  /// coloring_rules.additional_styles.insert(
  ///   "primary_action_button".into(),
  ///   "animate-slide_in pl-8 pr-8 shadow-md px-4 py-1 text-white bg-sky-500 hover:bg-sky-300 hover:text-gray-700".into()
  /// );
  /// ```
  pub additional_styles: HashMap<String, String>,
}