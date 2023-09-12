// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::{error, fmt, str};

use inquire::validator::Validation;
use inquire::CustomUserError;

// -------- //
// Fonction //
// -------- //

/// Invite l'utilisateur à saisir une entrée optionnelle depuis la console du
/// terminal.
pub fn optional<Output>(message: impl fmt::Display) -> Option<Output>
where
	Output: str::FromStr,
	Output::Err: Into<Box<dyn error::Error>>,
{
	let msg = format!("{message}:");
	let input = inquire::Text::new(&msg).prompt_skippable().ok()?;
	input.and_then(|value| Output::from_str(&value).ok())
}

/// Invite l'utilisateur à saisir une entrée optionnelle depuis la console du
/// terminal avec un validateur.
pub fn optional_with_validator<Output>(
	message: impl fmt::Display,
	validator: fn(&str) -> Result<Validation, CustomUserError>,
) -> Option<Output>
where
	Output: str::FromStr,
	Output::Err: Into<Box<dyn error::Error>>,
{
	let msg = format!("{message}:");
	let input = inquire::Text::new(&msg)
		.with_validator(validator)
		.prompt_skippable()
		.ok()?;
	input.and_then(|value| Output::from_str(&value).ok())
}
