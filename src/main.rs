//! # MMV utility
//!
//! `mmv` is an CLI utility for convinient for renaming and moving files in a completly new, convinient way. It supports basic regular pattern matching for moving numerous amount of files in 1 short command

mod cli;
mod controller;
mod filesystem;
mod matcher;
use controller::MassMoveController;

/// `mmv` entry point. See `Controller` documentaion for inside-view at the apllication infrastructure
fn main() {
    MassMoveController::new().run()
}
