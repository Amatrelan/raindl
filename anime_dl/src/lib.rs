#![warn(clippy::all)]
#![warn(clippy::as_conversions)]
#![warn(clippy::cargo_common_metadata)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::checked_conversions)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::create_dir)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::pedantic)]
// #![warn(clippy::unwrap_in_result)] // Need to find a way to fix some libraries so can use ?.
#![warn(clippy::verbose_file_reads)]
#![warn(clippy::wildcard_enum_match_arm)]
#![warn(clippy::wildcard_imports)]

pub mod error;
pub mod provider;
pub mod types;
