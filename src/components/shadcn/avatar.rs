//! Avatar component from `shadcn-ui`.
//!
//! Usage example:
//!
//! ```rust
//! use cc_ui_kit::prelude::*;
//! use cc_ui_kit::components::avatar::*;
//!
//! fn App(cx: Scope) -> Element {
//!   cx.render(rsx! {
//!     Avatar {
//!       AvatarImage {
//!         src: "https://github.com/shadcn.png",
//!       }
//!       AvatarFallback {
//!         "CN"
//!       }
//!     }
//!   })
//! }
//! ```

use crate::prelude::*;
use crate::utils::cn;

// Avatar root props
#[derive(Props)]
pub struct AvatarProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  children: Element<'a>,
}

// Avatar image props
#[derive(Props)]
pub struct AvatarImageProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  src: &'a str,
  #[props(optional)]
  alt: Option<&'a str>,
  #[props(optional)]
  onload: Option<EventHandler<'a, Event<ImageData>>>,
  #[props(optional)]
  onerror: Option<EventHandler<'a, Event<ImageData>>>,
}

// Avatar fallback props
#[derive(Props)]
pub struct AvatarFallbackProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  children: Element<'a>,
}

// Main Avatar component
pub fn Avatar<'a>(cx: Scope<'a, AvatarProps<'a>>) -> Element<'a> {
  let classes = cn(
    "relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full",
    cx.props.class,
  );

  cx.render(rsx! {
      div {
          class: "{classes}",
          &cx.props.children
      }
  })
}

// Avatar image component
pub fn AvatarImage<'a>(cx: Scope<'a, AvatarImageProps<'a>>) -> Element<'a> {
  let classes = cn("aspect-square h-full w-full", cx.props.class);

  cx.render(rsx! {
      img {
          class: "{classes}",
          src: cx.props.src,
          alt: cx.props.alt.unwrap_or(""),
          onload: move |evt| {
              if let Some(handler) = &cx.props.onload {
                  handler.call(evt);
              }
          },
          onerror: move |evt| {
              if let Some(handler) = &cx.props.onerror {
                  handler.call(evt);
              }
          },
      }
  })
}

// Avatar fallback component
pub fn AvatarFallback<'a>(cx: Scope<'a, AvatarFallbackProps<'a>>) -> Element<'a> {
  let classes = cn(
    "flex h-full w-full items-center justify-center rounded-full bg-muted",
    cx.props.class,
  );

  // If delayms is set, we could implement a delayed render here
  // For now, we'll just render immediately
  cx.render(rsx! {
      div {
          class: "{classes}",
          &cx.props.children
      }
  })
}

// Example usage
pub fn AvatarExample(cx: Scope) -> Element {
  cx.render(rsx! {
    Avatar {
      AvatarImage {
        src: "https://github.com/shadcn.png",
        alt: "User avatar",
        onerror: |_| { /* Handle error */ }
      }
      AvatarFallback {
        "CN"
      }
    }
  })
}
