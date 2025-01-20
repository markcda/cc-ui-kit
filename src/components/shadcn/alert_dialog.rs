//! Alert dialog element from `shadcn-ui`.
//!
//! Usage example:
//!
//! ```rust
//! use cc_ui_kit::prelude::*;
//! use cc_ui_kit::components::alert_dialog::*;
//!
//! fn App(cx: Scope) -> Element {
//!   let is_open = use_state(cx, || false);
//!
//!   cx.render(rsx! {
//!     AlertDialog {
//!       AlertDialogTrigger {
//!         is_dialog_open: is_open,
//!         "Open Dialog"
//!       }
//!       AlertDialogContent {
//!         is_dialog_open: is_open,
//!         AlertDialogHeader {
//!           AlertDialogTitle {
//!             "Are you absolutely sure?"
//!           }
//!           AlertDialogDescription {
//!             "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
//!           }
//!         }
//!         AlertDialogFooter {
//!           AlertDialogCancel {
//!             onclick: |_| {},
//!             is_dialog_open: is_open,
//!             "Cancel"
//!           }
//!           AlertDialogAction {
//!             onclick: |_| {},
//!             is_dialog_open: is_open,
//!             "Continue"
//!           }
//!         }
//!       }
//!     }
//!   })
//! }
//! ```

use crate::prelude::*;
use crate::utils::cn;

// Button variant types (simplified version of buttonVariants)
#[derive(Copy, Clone, PartialEq, Default)]
pub enum ButtonVariant {
  #[default]
  Default,
  Outline,
}

// Props for the main AlertDialog component
#[derive(Props)]
pub struct AlertDialogProps<'a> {
  children: Element<'a>,
}

// Props for content components
#[derive(Props)]
pub struct DialogContentProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  children: Element<'a>,
  is_dialog_open: &'a UseState<bool>,
}

#[derive(Props)]
pub struct ContentProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  children: Element<'a>,
}

// Props for action buttons
#[derive(Props)]
pub struct ActionProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  #[props(optional)]
  variant: Option<ButtonVariant>,
  onclick: EventHandler<'a, MouseEvent>,
  is_dialog_open: &'a UseState<bool>,
  children: Element<'a>,
}

#[derive(Props)]
pub struct AlertDialogTriggerProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  is_dialog_open: &'a UseState<bool>,
  children: Element<'a>,
}

// Main AlertDialog component
pub fn AlertDialog<'a>(cx: Scope<'a, AlertDialogProps<'a>>) -> Element<'a> {
  cx.render(rsx! {
    div {
      class: "relative",
      &cx.props.children
    }
  })
}

// Trigger component
pub fn AlertDialogTrigger<'a>(cx: Scope<'a, AlertDialogTriggerProps<'a>>) -> Element<'a> {
  let classes = cn(
    "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
    cx.props.class
  );

  cx.render(rsx! {
    button {
      class: "{classes}",
      onclick: move |_| { cx.props.is_dialog_open.set(true); },
      &cx.props.children
    }
  })
}

// Content wrapper component
pub fn AlertDialogContent<'a>(cx: Scope<'a, DialogContentProps<'a>>) -> Element<'a> {
  if !**cx.props.is_dialog_open {
    return None;
  }
  let classes = cn(
    "fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg",
    cx.props.class,
  );

  cx.render(rsx! {
    div {
      class: "fixed inset-0 z-50",
      // Overlay
      div {
        class: "fixed inset-0 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0",
        onclick: move |_| { cx.props.is_dialog_open.set(false); }
      },
      // Content
      div {
        class: "{classes}",
        role: "alertdialog",
        &cx.props.children
      }
    }
  })
}

// Header component
pub fn AlertDialogHeader<'a>(cx: Scope<'a, ContentProps<'a>>) -> Element<'a> {
  let classes = cn(
    "flex flex-col space-y-2 text-center sm:text-left",
    cx.props.class,
  );

  cx.render(rsx! {
    div {
      class: "{classes}",
      &cx.props.children
    }
  })
}

// Footer component
pub fn AlertDialogFooter<'a>(cx: Scope<'a, ContentProps<'a>>) -> Element<'a> {
  let classes = cn(
    "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2",
    cx.props.class,
  );

  cx.render(rsx! {
    div {
      class: "{classes}",
      &cx.props.children
    }
  })
}

// Title component
pub fn AlertDialogTitle<'a>(cx: Scope<'a, ContentProps<'a>>) -> Element<'a> {
  let classes = cn("text-lg font-semibold", cx.props.class);

  cx.render(rsx! {
    h2 {
      class: "{classes}",
      &cx.props.children
    }
  })
}

// Description component
pub fn AlertDialogDescription<'a>(cx: Scope<'a, ContentProps<'a>>) -> Element<'a> {
  let classes = cn("text-sm text-muted-foreground", cx.props.class);

  cx.render(rsx! {
    div {
      class: "{classes}",
      &cx.props.children
    }
  })
}

// Action button component
pub fn AlertDialogAction<'a>(cx: Scope<'a, ActionProps<'a>>) -> Element<'a> {
  let base_classes = match cx.props.variant.unwrap_or_default() {
    ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
    ButtonVariant::Outline => {
      "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
    }
  };
  let classes = cn(
    &format!("inline-flex items-center justify-center rounded-md px-4 py-2 text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 {}", base_classes),
    cx.props.class,
  );

  cx.render(rsx! {
    button {
      class: "{classes}",
      onclick: move |evt| cx.props.onclick.call(evt),
      &cx.props.children
    }
  })
}

// Cancel button component
pub fn AlertDialogCancel<'a>(cx: Scope<'a, ActionProps<'a>>) -> Element<'a> {
  let classes = cn(
    "mt-2 sm:mt-0 inline-flex items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-medium ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
    cx.props.class,
  );

  cx.render(rsx! {
    button {
      class: "{classes}",
      onclick: move |evt| {
        cx.props.is_dialog_open.set(false);
        cx.props.onclick.call(evt);
      },
      &cx.props.children
    }
  })
}

// Example usage
pub fn AlertDialogExample(cx: Scope) -> Element {
  let is_open = use_state(cx, || false);

  cx.render(rsx! {
    AlertDialog {
      AlertDialogTrigger {
        is_dialog_open: is_open,
        "Open Dialog"
      }
      AlertDialogContent {
        is_dialog_open: is_open,
        AlertDialogHeader {
          AlertDialogTitle {
            "Are you absolutely sure?"
          }
          AlertDialogDescription {
            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
          }
        }
        AlertDialogFooter {
          AlertDialogCancel {
            onclick: |_| {},
            is_dialog_open: is_open,
            "Cancel"
          }
          AlertDialogAction {
            onclick: |_| {},
            is_dialog_open: is_open,
            "Continue"
          }
        }
      }
    }
  })
}
