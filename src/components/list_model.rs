use dioxus::prelude::*;

use crate::hvalues::HValue;

pub fn ListModel<'a, T: Clone>(
  cx: Scope<'a>,
  values: HValue<'a, Vec<T>>,
  statement: impl Fn(&'a T) -> bool,
  list_gen: impl Fn(&'a T) -> LazyNodes<'a, 'a>,
  view_gen: impl Fn(&'a T) -> Option<LazyNodes<'a, 'a>>,
) -> Element<'a> {
  cx.render({
    let mut top_list = vec![];
    let mut bottom_list = vec![];
    let mut curr_element = None;

    let mut found = false;

    if let Some(vals) = values.read_ref() {
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