// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃                           __    _            ____  ______                 ┃
// ┃                    ____  / /_  (_)______  __/ __ \/ ____/                 ┃
// ┃                   / __ \/ __ \/ / ___/ / / / /_/ / /                      ┃
// ┃                  / /_/ / / / / (__  ) /_/ / _, _/ /___                    ┃
// ┃                 / .___/_/ /_/_/____/\__, /_/ |_|\____/                    ┃
// ┃                /_/                 /____/                                 ┃
// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
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

/// Invite l'utilisateur à saisir une entrée requise depuis la console du
/// terminal.
pub fn required<Output>(message: impl fmt::Display) -> Result<Output>
where
	Output: str::FromStr,
	Output::Err: Into<Box<dyn error::Error>>,
{
	let validator = |input: &str| {
		if input.trim().is_empty() {
			let msg = "L'entrée NE PEUT PAS être vide.";
			let custom_msg =
				<_ as Into<inquire::validator::ErrorMessage>>::into(msg);
			Ok(crate::Validation::Invalid(custom_msg))
		} else {
			Ok(crate::Validation::Valid)
		}
	};
	required_with_validator(message, validator)
}

/// Invite l'utilisateur à saisir une entrée requise depuis la console du
/// terminal avec un validateur.
pub fn required_with_validator<Output>(
	message: impl fmt::Display,
	validator: fn(&str) -> Result<Validation, CustomUserError>,
) -> Result<Output>
where
	Output: str::FromStr,
	Output::Err: Into<Box<dyn error::Error>>,
{
	let msg = format!("{message}:");

	let input = inquire::Text::new(&msg)
		.with_validator(validator)
		.prompt()
		.map(|value| Output::from_str(&value).map_err(|e| e.into()))??;

	Ok(input)
}
