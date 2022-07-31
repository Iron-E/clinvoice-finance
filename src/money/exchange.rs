use super::Money;
use crate::{Currency, Exchange, ExchangeRates};

impl Exchange for Money
{
	type Output = Self;

	/// The result will be [rounded](rust_decimal::Decimal::rescale) to two decimal places.
	///
	/// # See
	///
	/// * [`Exchange::exchange`]
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		// noop for same currency
		if self.currency == currency
		{
			return self;
		}

		let mut exchanged = self.amount * rates.index(&self.currency..&currency);
		exchanged.rescale(2);
		Self::Output {
			amount: exchanged,
			currency,
		}
	}
}

impl Exchange for &Money
{
	type Output = Money;

	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self::Output
	{
		(*self).exchange(currency, rates)
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::{Currency, ExchangeRates, Money};
	use crate::{Exchange, SAMPLE_EXCHANGE_RATES_CSV};

	#[test]
	fn exchange()
	{
		let exchange_rates = SAMPLE_EXCHANGE_RATES_CSV.parse::<ExchangeRates>().unwrap();

		let usd = Money::new(20_00, 2, Currency::Usd);

		let usd_to_jpy = usd.exchange(Currency::Jpy, &exchange_rates);
		assert_eq!(usd_to_jpy, Money::new(2195_95, 2, Currency::Jpy));

		// Assert round-trip works
		let usd_to_jpy_to_usd = usd_to_jpy.exchange(Currency::Usd, &exchange_rates);
		assert_eq!(usd, usd_to_jpy_to_usd);
	}
}
