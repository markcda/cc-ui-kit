use std::collections::HashMap;

pub struct CCDioxusV04ColoringRules {
  /// Tailwind-like class for primary color: text, decoration and accent.
  ///
  /// Example:
  /// ```rust
  ///
  /// ```
  pub primary_color: String,
  /// Tailwind-like class for secondary color: background.
  pub secondary_color: String,
  pub primary_color_hovered: String,
  pub secondary_color_hovered: String,
  pub primary_color_selected: String,
  pub secondary_color_selected: String,
  /// Additional styles for components.
  ///
  /// Usage:
  ///
  /// ```rust,ignore
  /// use cc_ui_kit::components::coloring_rules::CCDioxusV04ColoringRules;
  ///
  /// let mut coloring_rules = CCDioxusV04ColoringRules { ... };
  /// coloring_rules.additional_styles.insert(
  ///   "primary_action_button".into(),
  ///   "animate-slide_in pl-8 pr-8 shadow-md px-4 py-1 text-white bg-sky-500 hover:bg-sky-300 hover:text-gray-700".into()
  /// );
  /// ```
  pub additional_styles: HashMap<String, String>,
}