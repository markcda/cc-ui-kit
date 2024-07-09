use crate::prelude::*;

/// Компонент модели списка.
///
/// Перечисляет элементы списка; при нажатии на один из элементов отображает модель текущего элемента
/// сразу после выбранного элемента списка.
///
/// Пример:
///
/// ```rust
/// use cc_ui_kit::prelude::*;
/// use cc_ui_kit::components::list_model::ListModel;
///
/// pub fn Component(cx: Scope) -> Element {
///   let selected_source = use_state(cx, || 0);
///
///   cx.render({
///     let sources = vec![1,2,3];
///     rsx! {
///       ListModel(
///         cx,
///         &sources,                            // Список всех данных
///         |val| { **selected_source == *val }, // Условие выбора: например, если `sources` - это вектор сложных объектов
///         |val| { rsx!(div { "{val}" }) },     // Отображение каждого элемента данных
///         |_| {                                // Отображение выбранного элемента, если он есть
///           if **selected_source != 0 {
///             Some(rsx!{ div { "Выбранный источник: {selected_source}" } })
///           } else {
///             None
///           }
///         }
///       )
///      }
///   })
/// }
/// ```
pub fn ListModel<'a, T: Clone + 'static>(
  cx: Scope<'a>,
  values: impl Into<HValue<'a, Vec<T>>>,
  statement: impl Fn(&'a T) -> bool,
  list_gen: impl Fn(&'a T) -> LazyNodes<'a, 'a>,
  view_gen: impl Fn(&'a T) -> Option<LazyNodes<'a, 'a>>,
) -> Element<'a> {
  cx.render({
    let mut top_list = vec![];
    let mut bottom_list = vec![];
    let mut curr_element = None;

    let mut found = false;

    if let Some(vals) = values.into().read_ref() {
      for val in vals.iter() {
        if !found {
          top_list.push(list_gen(&val));
          if statement(&val) {
            found = true;
            curr_element = view_gen(&val);
          }
        } else {
          bottom_list.push(list_gen(&val));
        }
      }
    }

    rsx! {
      top_list.into_iter(),
      curr_element,
      bottom_list.into_iter(),
    }
  })
}
