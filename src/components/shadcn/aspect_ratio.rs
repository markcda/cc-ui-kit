//! AspectRatio element from `shadcn-ui`.
//!
//! Usage example:
//!
//! ```rust
//! use cc_ui_kit::prelude::*;
//! use cc_ui_kit::components::aspect_ratio::*;
//!
//! fn App(cx: Scope) -> Element {
//!   cx.render(rsx! {
//!     div {
//!       class: "w-[450px]",
//!       AspectRatio {
//!         ratio: 16.0 / 9.0,
//!         img {
//!           src: "...",
//!           alt: "Image",
//!           class: "rounded-md object-cover"
//!         }
//!       }
//!     }
//!   })
//! }
//! ```

use crate::prelude::*;
use crate::utils::cn;

#[derive(Props)]
pub struct AspectRatioProps<'a> {
  #[props(optional)]
  class: Option<&'a str>,
  ratio: f64,
  children: Element<'a>,
}

pub fn AspectRatio<'a>(cx: Scope<'a, AspectRatioProps<'a>>) -> Element<'a> {
  let padding_bottom = (1.0 / cx.props.ratio * 100.0).to_string();
  let classes = cn("relative w-full", cx.props.class);

  cx.render(rsx! {
    div {
      class: "{classes}",
      // Create pseudo-element for maintaining aspect ratio
      style: "padding-bottom: {padding_bottom}%",
      // Container for actual content
      div {
        class: "absolute inset-0",
        &cx.props.children
      }
    }
  })
}

// Example usage
pub fn AspectRatioExample(cx: Scope) -> Element {
  cx.render(rsx! {
    div {
      class: "w-[450px]",
      AspectRatio {
        ratio: 16.0 / 9.0,
        img {
          src: "https://d.rsms.me/stuff/colorful-circles.jpeg",
          alt: "Image",
          class: "rounded-md object-cover"
        }
      }
    }
  })
}
