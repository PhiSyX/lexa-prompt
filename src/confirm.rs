// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::fmt;

use crate::choices;

// ----------- //
// Énumération //
// ----------- //

choices! {
	#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
	pub enum Bool
	{
		False,
		True,

		N,
		Y,

		No,
		Yes,
	}
}

// -------------- //
// Implémentation //
// -------------- //

impl Bool
{
	/// Vérifie que la valeur de [&Bool](Self) soit vraie.
	pub fn is_true(&self) -> bool
	{
		matches!(self, Self::True | Self::Y | Self::Yes)
	}

	/// Vérifie que la valeur de [&Bool](Self) soit fausse.
	pub fn is_false(&self) -> bool
	{
		matches!(self, Self::False | Self::N | Self::No)
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<Bool> for bool
{
	fn from(value: Bool) -> Self
	{
		value.is_true()
	}
}

// -------- //
// Fonction //
// -------- //

/// Invite l'utilisateur à confirmer une question, par oui (y) ou non (n).
pub fn confirm(question: impl fmt::Display) -> bool
{
	let ask = format!("{question} ?");

	let Ok(b) = inquire::Confirm::new(&ask).prompt() else {
		return false;
	};

	b
}
