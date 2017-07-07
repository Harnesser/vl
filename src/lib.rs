//! Board Support Crate for the STM32VLDISCOVERY
//!
//! # Features
//!
//! - Periodic timer
//! - Serial interface
//! - User LEDs
//!
//! # Usage
//!
//! Follow `cortex-m-quickstart` [instructions][i], and add this crate as a
//! dependency and remove the `memory.x` linker script and the `build.rs` build
//! script file as part of the configuration of the quickstart crate.
//!
//! [i]: https://docs.rs/cortex-m-quickstart/0.1.8/cortex_m_quickstart/

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub extern crate stm32f100xx;

extern crate cast;

pub mod led;
pub mod serial;
pub mod timer;
pub mod button;

// non-board stuff
//pub mod lcd;
//pub mod rotary_encoder;

mod frequency;
