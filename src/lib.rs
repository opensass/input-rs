#![doc(
    html_logo_url = "https://github.com/opensass/input-rs/blob/main/assets/logo.png",
    html_favicon_url = "https://github.com/opensass/input-rs/blob/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub(crate) mod countries;
#[cfg(feature = "yew")]
pub mod yew;
