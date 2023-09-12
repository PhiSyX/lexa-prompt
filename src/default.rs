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

use crate::Result;

// -------- //
// Fonction //
// -------- //

/// Invite l'utilisateur à saisir une entrée optionnelle depuis la console du
/// terminal.
pub fn default<Output>(
	message: impl fmt::Display,
	default: impl fmt::Display,
) -> Result<Output>
where
	Output: str::FromStr,
	Output::Err: Into<Box<dyn error::Error + Send + Sync>>,
{
	default_with_validator(message, default, |_: &str| Ok(Validation::Valid))
}

/// Invite l'utilisateur à saisir une entrée optionnelle depuis la console du
/// terminal avec un validateur.
pub fn default_with_validator<Output>(
	message: impl fmt::Display,
	default: impl fmt::Display,
	validator: fn(&str) -> Result<Validation, CustomUserError>,
) -> Result<Output>
where
	Output: str::FromStr,
	Output::Err: Into<Box<dyn error::Error + Send + Sync>>,
{
	let msg = format!("{message}:");
	let default = format!("{default}");

	let input = inquire::Text::new(&msg)
		.with_default(&default)
		.with_validator(validator)
		.prompt()
		.map(|value| Output::from_str(&value).map_err(|e| e.into()))??;

	Ok(input)
}
