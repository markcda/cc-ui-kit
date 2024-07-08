use cc_utils::errors::CliError;
use dioxus::prelude::*;
use dioxus_v04_optional_hooks::FutureHook;

pub enum HValue<'a, T: Clone + 'static> {
  State(&'a UseState<T>),
  Reference(&'a T),
  Future((&'a FutureHook<'a, T, CliError>, bool)),
}

impl<'a, T: Clone + 'static> HValue<'a, T> {
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
}