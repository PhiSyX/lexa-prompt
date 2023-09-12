// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

#![feature(return_position_impl_trait_in_trait)]

mod choice;
mod confirm;
mod default;
mod error;
mod for_loop;
mod interface;
mod macros;
mod optional;
mod required;

pub use inquire::validator::Validation;
pub use lexa_prompt_macro::Prompt;

pub use self::choice::choice;
pub use self::confirm::{confirm, Bool};
pub use self::default::{default, default_with_validator};
pub use self::error::Error;
pub use self::for_loop::for_loop;
pub use self::interface::{Choice, Prompt};
pub use self::optional::{optional, optional_with_validator};
pub use self::required::{required, required_with_validator};

// ---- //
// Type //
// ---- //

pub type Result<T, E = Error> = std::result::Result<T, E>;
