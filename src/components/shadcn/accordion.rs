//! Accordion element from `shadcn-ui`.
//! 
//! Usage example:
//! 
//! ```rust
//! use cc_ui_kit::prelude::*;
//! use cc_ui_kit::components::accordion::*;
//! 
//! fn App() -> Element {
//!   let is_open = use_signal(|| false);
//!
//!   rsx! {
//!     Accordion {
//!       AccordionItem {
//!         AccordionTrigger {
//!           is_open: is_open,
//!           "Is it accessible?"
//!         }
//!         AccordionContent {
//!           is_open: is_open,
//!           "Yes. It adheres to the WAI-ARIA design pattern."
//!         }
//!       }
//!     }
//!   }
//! }
//! ```

use dioxus_free_icons::{Icon, icons::hi_solid_icons::HiChevronDown};

use crate::prelude::*;
use crate::utils::cn;

#[derive(Clone, Debug, Props)]
pub struct AccordionItemProps<'a> {
  class: Option<String>,
  children: Element<'a>,
}

#[derive(Clone, Debug, Props)]
pub struct AccordionTriggerProps<'a> {
  class: Option<String>,
  is_open: &'a UseState<bool>,
  children: Element<'a>,
}

#[derive(Clone, Debug, Props)]
pub struct AccordionContentProps<'a> {
  class: Option<String>,
  is_open: &'a UseState<bool>,
  children: Element<'a>,
}

#[derive(Clone, Debug, Props)]
pub struct AccordionProps<'a> {
  children: Element<'a>,
}

#[component]
pub fn Accordion<'a>(cx: Scope<'a, AccordionProps<'a>>) -> Element<'a> {
  cx.render({
    rsx! {
      &cx.props.children
    }
  })
}

#[component]
pub fn AccordionItem<'a>(cx: Scope<'a, AccordionItemProps<'a>>) -> Element<'a> {
  cx.render({
    let acc_item = cn(&["border-b", cx.props.class.as_ref().unwrap_or(&String::new())]);
    
    rsx! {
      div {
        class: "{acc_item}",
        &cx.props.children
      }
    }
  })
}

pub fn AccordionTrigger<'a>(cx: Scope<'a, AccordionTriggerProps<'a>>) -> Element<'a> {
  cx.render({
    let div_class = cn(&[
      "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline",
      cx.props.class.as_ref().unwrap_or(&String::new()),
    ]);
    let icon_class = cn(&[
      "h-4 w-4 shrink-0 transition-transform duration-200",
      if **cx.props.is_open { "rotate-180" } else { "" },
    ]);
      
    rsx! {
      div {
        class: "flex",
        button {
          class: "{div_class}",
          onclick: move |_| cx.props.is_open.set(!**cx.props.is_open),
          &cx.props.children,
          Icon {
            icon: HiChevronDown,
            class: "{icon_class}"
          }
        }
      }
    }
  })
}

pub fn AccordionContent<'a>(cx: Scope<'a, AccordionContentProps<'a>>) -> Element<'a> {
  cx.render({
    let inner_div_class = cn(&["pb-4 pt-0", cx.props.class.as_ref().unwrap_or(&String::new())]);
    let state = if **cx.props.is_open { "open" } else { "closed" };

    rsx! {
      div {
        "data-state": state,
        class: "overflow-hidden text-sm transition-all data-[state=closed]:max-h-0 data-[state=open]:max-h-fit data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down",
        style: "height: var(--cc-ui-kit-content-height)",
        div {
          class: "{inner_div_class}",
          &cx.props.children
        }
      }
    }
  })
}
