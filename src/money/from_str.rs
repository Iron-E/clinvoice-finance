use core::str::FromStr;

use super::Money;
use crate::{Error, Result};

impl FromStr for Money
{
	type Err = Error;

	fn from_str(s: &str) -> Result<Self>
	{
		let (amount, currency) = {
			let new_error = |field: &str| -> Error {
				Error::Decode(
					format!(r#""{s}" into money"#),
					format!("there was no {field}"),
				)
			};

			let mut split = s.split(' ');
			(
				split.next().ok_or_else(|| new_error("amount"))?,
				split.next().ok_or_else(|| new_error("currency"))?,
			)
		};

		Ok(Money {
			amount: amount.parse()?,
			currency: currency.parse()?,
		})
	}
}