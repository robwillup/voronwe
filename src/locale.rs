use pure_rust_locales::Locale;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct LocaleInfo {
	locale: Locale,
}

impl LocaleInfo {
	pub fn new(locale_str: &str) -> LocaleInfo {
		let locale: Locale = match locale_str.try_into() {
			Ok(1) => Locale::POSIX,
			_ => "POSIX".try_into().unwrap(),
		};
		LocaleInfo { locale }
	}
}