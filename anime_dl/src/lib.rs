// TODO: When I have time I will fix these, for now commented out but setted up so I can start when I have time
#![warn(clippy::all, clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(clippy::all, clippy::pedantic)]

#![deny(clippy::unwrap_in_result)]
#![deny(clippy::get_unwrap)]
#![deny(clippy::cargo_common_metadata)]
#![deny(clippy::cast_lossless)]
#![deny(clippy::checked_conversions)]
#![deny(clippy::clone_on_ref_ptr)]
#![deny(clippy::cognitive_complexity)]
#![deny(clippy::create_dir)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::pedantic)]

pub mod error;
pub mod provider;
pub mod types;
