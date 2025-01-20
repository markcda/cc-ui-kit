//! Alert element from `shadcn-ui`.
//!
//! Usage example:
//!
//! ```rust
//! use cc_ui_kit::prelude::*;
//! use cc_ui_kit::components::alert::*;
//!
//! fn App(cx: Scope) -> Element {
//!   cx.render(rsx! {
//!     Alert {
//!       variant: AlertVariant::Default,
//!       AlertTitle {
//!         "Success!"
//!       }
//!       AlertDescription {
//!         "Your action has been completed successfully."
//!       }
//!     }
//!     
//!     Alert {
//!       variant: AlertVariant::Destructive,
//!       AlertTitle {
//!         "Error"
//!       }
//!       AlertDescription {
//!         "Something went wrong. Please try again."
//!       }
//!     }
//!   })
//! }
//! ```

use crate::prelude::*;
use crate::utils::cn;

// Define variants for the alert
#[derive(PartialEq)]
pub enum AlertVariant {
  Default,
  Destructive,
}

// Alert component props
#[derive(Props)]
pub struct AlertProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  #[props(optional)]
  variant: Option<AlertVariant>,
  children: Element<'a>,
}

// Alert title props
#[derive(Props)]
pub struct AlertTitleProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  children: Element<'a>,
}

// Alert description props
#[derive(Props)]
pub struct AlertDescriptionProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  children: Element<'a>,
}

// Alert component
pub fn Alert<'a>(cx: Scope<'a, AlertProps<'a>>) -> Element<'a> {
  let base_classes = "relative w-full rounded-lg border px-4 py-3 text-sm [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground [&>svg~*]:pl-7";

  let variant_classes = match cx.props.variant.as_ref().unwrap_or(&AlertVariant::Default) {
    AlertVariant::Default => "bg-background text-foreground",
    AlertVariant::Destructive => {
      "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive"
    }
  };

  let classes = cn(
    &format!("{} {}", base_classes, variant_classes),
    cx.props.class,
  );

  cx.render(rsx! {
    div {
      role: "alert",
      class: "{classes}",
      &cx.props.children
    }
  })
}

// Alert title component
pub fn AlertTitle<'a>(cx: Scope<'a, AlertTitleProps<'a>>) -> Element<'a> {
  let classes = cn(
    "mb-1 font-medium leading-none tracking-tight",
    cx.props.class,
  );

  cx.render(rsx! {
    h5 {
      class: "{classes}",
      &cx.props.children
    }
  })
}

// Alert description component
pub fn AlertDescription<'a>(cx: Scope<'a, AlertDescriptionProps<'a>>) -> Element<'a> {
  let classes = cn("text-sm [&_p]:leading-relaxed", cx.props.class);

  cx.render(rsx! {
    div {
      class: "{classes}",
      &cx.props.children
    }
  })
}

// Example usage:
pub fn AlertExample(cx: Scope) -> Element {
  cx.render(rsx! {
    Alert {
      variant: AlertVariant::Default,
      AlertTitle {
        "Success!"
      }
      AlertDescription {
        "Your action has been completed successfully."
      }
    }

    Alert {
      variant: AlertVariant::Destructive,
      AlertTitle {
        "Error"
      }
      AlertDescription {
        "Something went wrong. Please try again."
      }
    }
  })
}
