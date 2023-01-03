mod locale;

const REFORM_YEAR: u32 = 1099;

// SPECIAL_LEAP_YEARS are years before the REFORM_YEAR that are divisible by 100, but not by 400.
const SPECIAL_LEAP_YEARS: u32 = (REFORM_YEAR / 100) - (REFORM_YEAR / 400);

const MONTHS: usize = 12;
const WEEKDAYS: u32 = 7;

const COLUMN: usize = 3;
const ROWS: usize = 4;
const ROW_SIZE: usize = 7;

static TOKEN: &str = "\n";

fn is_leap_year(year: u32) -> bool {
	if year <= REFORM_YEAR {
		return year % 4 == 0;
	}
	(year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}

fn count_leap_years(year: u32) -> u32 {
	if year <= REFORM_YEAR {
		(year - 1) / 4
	} else {
		((year - 1) / 4) - ((year - 1) / 100) + ((year - 1) / 100) + SPECIAL_LEAP_YEARS
	}
}

fn days_by_year(year: u32) -> u32 {
	if year < 1 {
		0
	} else {
		(year - 1) * 365 + count_leap_years(year)
	}
}

fn days_by_month(year: u32) -> Vec<u32> {
	let feb_day: u32 = if is_leap_year(year) { 29 } else { 28 };
	vec![0, 31, feb_day, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

fn get_days_accumulated_by_month(year: u32) -> (Vec<u32>, Vec<u32>) {
	let mut count = 0;
	let mut accum = Vec::new();
	let days: Vec<u32> = days_by_month(year);

	(0..MONTHS + 1).for_each(|i| {
		count += days[i];
		accum.push(count);
	});
	(accum, days)
}

fn days_by_date(
	day: u32,
	month: usize,
	year: u32,
	months_memoized: Vec<u32>,
	year_memoized: u32,
) -> u32 {
	day + (if month > 1 {
		months_memoized[month - 1]
	} else {
		0
	}) + (if year > 1 { year_memoized } else { 0 })
}

fn first_day_printable(day_year: u32, starting_day: u32) -> String {
	let mut spaces: String = "".to_string();
	let mut printable = format!("");

	if (day_year - starting_day) % WEEKDAYS == 0 {
		printable.push_str("           ");
	}
	for i in 2..WEEKDAYS {
		spaces += &"   ".to_string();
		if (day_year - starting_day) % WEEKDAYS == i {
			printable.push_str(spaces.as_str());
			break;
		}
	}
	printable
}

fn remain_day_printable(day: u32, day_year: u32, starting_day: u32) -> String {
	let base = if ((day_year - starting_day) % WEEKDAYS) == 0 {
		format!("{:3}{}", day, TOKEN)
	} else {
		String::default()
	};

	let complement = (1..WEEKDAYS)
		.find_map(|i| ((day_year - starting_day) % WEEKDAYS == i).then(|| format!("{:3}", day)))
		.unwrap_or_default();

	format!("{}{}", base, complement)
}

fn body_printable(
	year: u32,
	month: usize,
	days: u32,
	months_memoized: Vec<u32>,
	year_memoized: u32,
	starting_day: u32
) -> Vec<String> {
	let mut result = Vec::<String>::new();
	let mut result_days = format!("");

	// Display month formatted
	(1..days + 1).for_each(|day| {
		if day == 1 {
			let first_day = days_by_date(1, month, year, months_memoized.clone(), year_memoized);
			result_days.push_str(&first_day_printable(first_day, starting_day))
		}
		let day_year = days_by_date(day, month, year, months_memoized.clone(), year_memoized);
		result_days.push_str(&remain_day_printable(day, day_year, starting_day))
	});

	// Lines splitted by '\n' TOKEN
	result_days
		.split(TOKEN)
		.collect::<Vec<&str>>()
		.into_iter()
		.for_each(|i| result.push(i.to_string()));

	// All body should have at least 6 lines
	let len = result.len();
	if len <= 6 {
		let spaces = 21 - result[len - 1].len();
		if result[len - 1].len() < 20 {
			for _i in 0..spaces {
				result[len - 1] += " "
			}
		}
		result.push("                           ".to_string())
	}
	result
}

fn month_printable(
	year: u32,
	month: usize,
	days: u32,
	months_memoized: Vec<u32>,
	year_memoized: u32,
	starting_day: u32,
	month_names: Vec<String>,
	week_name: Vec<String>,
) -> Vec<String> {
	let mut result = Vec::<String>::new();
	let body = body_printable(
		year,
		month,
		days,
		months_memoized,
		year_memoized,
		starting_day
	);
	let month_name = &month_names[month - 1];
	result.push(format!(" {:^20}", month_name));
	let header = circular_week_name(week_name, starting_day as usize);
	result.push(header);

	body.into_iter().for_each(|item| {
		result.push(item);
	});
	result
}

fn circular_week_name(week_name: Vec<String>, idx: usize) -> String {
	let mut s = " ".to_string();
	let mut i = idx;

	while i < ROW_SIZE + idx {
		if i == (ROW_SIZE - 1) + idx {
			s.push_str(week_name[i % ROW_SIZE].as_str());
		}
		i += 1
	}
	s.to_string()
}

pub fn calendar(year: u32, locale_str: &str, starting_day: u32) -> Vec<Vec<Vec<String>>> {
	let mut rows: Vec<Vec<Vec<String>>> = vec![vec![vec![String::from("")]; COLUMN]; ROWS];
	let mut row_counter = 0;
	let mut column_counter = 0;
	let (months_memoized, months) = get_days_accumulated_by_month(year);
	let year_memoized = days_by_year(year);
	let locale_info = locale::LocaleInfo::new(locale_str);

	(1..MONTHS + 1).for_each(|month| {
		rows[row_counter][column_counter] = month_printable(
			year,
			month,
			months[month],
			months_memoized.clone(),
			year_memoized,
			starting_day,
			locale_info.month_names(),
			locale_info.week_day_names(),
		);
		column_counter = month % COLUMN;
		if column_counter == 0 {
			row_counter += 1;
		}
	});
	rows
}

pub fn display(year: u32, locale_str: &str, starting_day: u32) {
	let rows = calendar(year, starting_day);
}