macro_rules! regex {
	($re:literal $(,)?) => {{
		::once_cell::sync::Lazy::<::regex::Regex>::new(|| {
			::regex::RegexBuilder::new($re)
				.case_insensitive(true)
				.ignore_whitespace(true)
				.build()
				.expect(&format!("Unable to build regex from `{}`.", $re))
		})
	}};
}

pub(crate) use regex;
