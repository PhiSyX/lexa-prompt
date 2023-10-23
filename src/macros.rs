// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

// ----- //
// Macro //
// ----- //

#[macro_export]
macro_rules! choices {
	(
		$(#[$derive:meta])*
		pub enum $enum:ident
		{
			$(
				$(#[$attr:meta])*
				$variant:ident,
			)*
		}
	) => {
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
$(#[$derive])*
pub enum $enum
{
	$(
		$(#[$attr])*
		$variant
	),*
}

impl $crate::Choice for $enum
{
	fn choices() -> &'static [&'static str]
	{
		&[
			$(stringify!($variant)),*
		]
	}
}

impl ::std::str::FromStr for $enum
{
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		$(
			if s.to_lowercase() == stringify!($variant).to_lowercase() {
				return Ok(Self::$variant);
			}
		)*

		let choices = <Self as $crate::Choice>::choices().join(", ");

		return Err(
			format!(
				"La valeur « {s} » est invalide. \
				Valeur attendue: {choices}",
			)
		);
	}
}

impl ::std::fmt::Display for $enum
{
	fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
	{
		let choice = match self {
			$( | Self::$variant => stringify!($variant) ),*
		};
		write!(f, "{}", choice)
	}
}
	};
}
