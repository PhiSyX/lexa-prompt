/*
 * @author Mike 'PhiSyX' S. (https://github.com/PhiSyX)
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{error, fmt, str};

use inquire::validator::Validation;
use inquire::CustomUserError;

use crate::confirm;

// -------- //
// Fonction //
// -------- //

pub fn for_loop<Output>(
	message: impl fmt::Display,
	validator: fn(&str) -> Result<Validation, CustomUserError>,
) -> Vec<Output>
where
	Output: str::FromStr,
	Output::Err: Into<Box<dyn error::Error + Send + Sync>>,
	Output::Err: fmt::Display,
{
	let mut responses = Vec::default();

	loop {
		let msg = format!("{message}:");

		let response = match inquire::Text::new(&msg)
			.with_validator(validator)
			.prompt()
			.and_then(|s| {
				s.parse::<Output>().map_err(|err| {
					let custom_err = <_ as Into<CustomUserError>>::into(err);
					inquire::InquireError::Custom(custom_err)
				})
			}) {
			| Ok(response) => response,
			| Err(err) => {
				log::error!("{err}");
				break;
			}
		};

		responses.push(response);

		if !confirm("  Continuer") {
			break;
		}
	}

	responses
}
