// Utility function to combine classes
pub fn cn(base: &str, additional: Option<String>) -> String {
  match additional {
    Some(class) if !class.is_empty() => format!("{} {}", base, class),
    _ => base.to_string(),
  }
}

pub fn dark_theme() -> thaw::Theme {
  let mut theme = thaw::Theme::dark();
  theme.color.color_brand_background = "#fafafa".to_string();
  theme.color.color_brand_background_hover = "#fafafae6".to_string();
  theme.color.color_brand_background_pressed = "#fafafae6".to_string();
  theme.color.color_neutral_background_1 = "#08080a".to_string();
  theme.color.color_neutral_foreground_on_brand = "#18181b".to_string();
  theme.color.color_neutral_foreground_2_brand_hover = "#fafafa".to_string();
  theme.color.color_neutral_foreground_2_brand_pressed = "#fafafae6".to_string();
  theme.color.color_neutral_foreground_2_brand_selected = "#fafafa".to_string();
  theme
}

pub fn light_theme() -> thaw::Theme {
  let mut theme = thaw::Theme::light();
  theme.color.color_brand_background = "#17171a".to_string();
  theme.color.color_brand_background_hover = "#17171ae6".to_string();
  theme.color.color_brand_background_pressed = "#17171ae6".to_string();
  theme.color.color_neutral_foreground_on_brand = "#fafafa".to_string();
  theme.color.color_neutral_foreground_2_brand_hover = "#18181b".to_string();
  theme.color.color_neutral_foreground_2_brand_pressed = "#18181be6".to_string();
  theme.color.color_neutral_foreground_2_brand_selected = "#18181b".to_string();
  theme
}
