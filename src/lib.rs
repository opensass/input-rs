#![doc(
    html_logo_url = "https://github.com/opensass/input-rs/blob/main/assets/logo.png",
    html_favicon_url = "https://github.com/opensass/input-rs/blob/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub(crate) mod countries;
#[cfg(feature = "dio")]
pub mod dioxus;
#[cfg(feature = "lep")]
pub mod leptos;
#[cfg(feature = "yew")]
pub mod yew;
