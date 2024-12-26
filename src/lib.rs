#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/input-rs/refs/heads/main/assets/logo-new.png",
    html_favicon_url = "https://raw.githubusercontent.com/opensass/input-rs/refs/heads/main/assets/favicon.png"
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
