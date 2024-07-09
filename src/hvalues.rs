//! Специальный враппер над разными видами ссылок на значения - мастхэв для Dioxus-а.

use cc_utils::errors::CliError;
use dioxus::prelude::*;
use dioxus_v04_optional_hooks::FutureHook;

/// Враппер над разными видами ссылок на значения.
///
/// Используется в компонентах для Dioxus, чтобы упростить чтение данных при рендеринге DOM.
#[derive(Clone)]
pub enum HValue<'a, T: Clone + 'static> {
  /// Стейт из Dioxus 0.4.0-0.4.3
  State(&'a UseState<T>),
  /// Ссылка на значение
  Reference(&'a T),
  /// Ожидаемое замыкание из `dioxus_v04_optional_hooks` и правило его чтения (по умолчанию - `false`).
  /// 
  /// Для деталей - см. `dioxus_v04_optional_hooks::FutureHook`.
  Future((&'a FutureHook<'a, T, CliError>, bool)),
}

impl<'a, T: Clone + 'static> Copy for HValue<'a, T> {}

impl<'a, T: Clone + 'static> HValue<'a, T> {
  /// Читает ссылку.
  ///
  /// Пример:
  /// ```rust
  /// use cc_ui_kit::prelude::*;
  ///
  /// pub fn Component<'a>(cx: Scope<'a>, hv: HValue<'a, String>) -> Element<'a> {
  ///   cx.render({
  ///     if let Some(val) = hv.read_ref() { // читаем ссылку вне зависимости от того, что в ней находится
  ///       rsx! { div { "{val}" } }
  ///     } else {
  ///       rsx!(div {})
  ///     }
  ///   })
  /// }
  ///
  /// pub fn Root(cx: Scope) -> Element {
  ///   let some_state = use_state(cx, || "str1".into());
  ///   const CONST_STR: String = "str2".into();
  ///
  ///   let hv1 = HValue::State(some_state);
  ///   let hv2 = HValue::Reference(&CONST_STR);
  ///
  ///   cx.render(rsx! {
  ///     Component(cx, hv1),
  ///     Component(cx, hv2),
  ///   })
  /// }
  /// ```
  pub fn read_ref(&self) -> Option<&'a T> {
    match self {
      HValue::Reference(val) => Some(val),
      HValue::State(state) => Some(*state),
      HValue::Future((future, read_rule)) => future.read(*read_rule),
    }
  }

  pub fn is_state(&self) -> bool {
    if let HValue::State(_) = self { true } else { false }
  }

  pub fn is_reference(&self) -> bool {
    if let HValue::Reference(_) = self { true } else { false }
  }

  pub fn is_future(&self) -> bool {
    if let HValue::Future(_) = self { true } else { false }
  }

  /// Прочитать ссылку без проверки.
  ///
  /// Выполнять `read_ref_unchecked` на варианте `Future` без проверки готовности её состояния **не допускается**!
  pub fn read_ref_unchecked(&self) -> &'a T {
    match self {
      HValue::Reference(val) => val,
      HValue::State(state) => *state,
      HValue::Future(_) => panic!("Не допускается выполнять `read_ref_unchecked` на варианте `Future` без проверки готовности её состояния!"),
    }
  }
}

impl<'a, T: Clone> From<&'a UseState<T>> for HValue<'a, T> {
  fn from(value: &'a UseState<T>) -> Self {
    Self::State(value)
  }
}

impl<'a, T: Clone> From<&'a T> for HValue<'a, T> {
  fn from(value: &'a T) -> Self {
    Self::Reference(value)
  }
}

impl<'a, T: Clone> From<&'a FutureHook<'a, T, CliError>> for HValue<'a, T> {
  fn from(value: &'a FutureHook<'a, T, CliError>) -> Self {
    Self::Future((value, false))
  }
}
