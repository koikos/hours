use std::fmt;
use std::str::FromStr;

use regex::Regex;
use simple_error::SimpleError;

type Decimal = f64;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Time {
    hours: u16,
    minutes: u8,
    seconds: u8,
}

impl Time {
    pub fn from(time: &String) -> Result<Time, SimpleError> {
        let re_hhhmmss = Regex::new(r"^(?P<h>\d*):(?P<m>\d*):?(?P<s>\d*)$").unwrap();
        let re_hhhdddd = Regex::new(r"^\d*[,.]?\d*$").unwrap();

        return if re_hhhmmss.is_match(time) {
            let caps = re_hhhmmss.captures(&time).unwrap();
            let time = Time::parse_hhhmmss(&caps["h"], &caps["m"], &caps["s"]);
            Ok(time)
        } else if re_hhhdddd.is_match(time) {
            Ok(Time::parse_hhhdddd(&time))
        } else {
            Err(SimpleError::new("Couldn't parse given time."))
        };
    }

    pub fn from_decimal(mut time: Decimal) -> Time {
        let hours = Decimal::floor(time);
        time = (time - hours) * 60.0;
        let minutes = Decimal::round(time);
        time = (time - minutes) * 60.0;
        let seconds = Decimal::round(time);
        return Time {
            hours: hours as u16,
            minutes: minutes as u8,
            seconds: seconds as u8,
        };
    }

    pub fn to_decimal(&self) -> Decimal {
        let hours = Decimal::from(self.hours);
        let minutes = Decimal::from(self.minutes);
        let seconds = Decimal::from(self.seconds);
        return hours + minutes / 60.0 + seconds / 3600.0;
    }

    fn parse_hhhmmss(hours: &str, minutes: &str, seconds: &str) -> Time {
        Time::normalize(
            hours.parse::<u16>().unwrap_or(0_u16),
            minutes.parse::<u16>().unwrap_or(0_u16),
            seconds.parse::<u16>().unwrap_or(0_u16),
        )
    }

    fn parse_hhhdddd(time: &String) -> Time {
        let fixed_time = Time::fix_comma(&time);
        let time_f64 = fixed_time.parse::<Decimal>().unwrap();
        return Time::from_decimal(time_f64);
    }

    fn normalize(hours: u16, minutes: u16, seconds: u16) -> Time {
        let mut minutes_u32 = minutes as u32 + u32::div_euclid(seconds as u32, 60);
        let seconds_u32 = u32::rem_euclid(seconds as u32, 60);

        let hours_u32 = hours as u32 + u32::div_euclid(minutes_u32, 60);
        minutes_u32 = u32::rem_euclid(minutes_u32, 60);

        return Time {
            hours: hours_u32 as u16,
            minutes: minutes_u32 as u8,
            seconds: seconds_u32 as u8,
        };
    }

    fn fix_comma(time: &str) -> std::borrow::Cow<'_, str> {
        let re = Regex::new(r"[,]").unwrap();
        return re.replace(&time, ".");
    }
}

impl FromStr for Time {
    type Err = SimpleError;

    fn from_str(time: &str) -> Result<Time, SimpleError> {
        return Time::from(&String::from(time));
    }
}

impl fmt::Display for Time {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "{}:{:0>2}:{:0>2}",
            self.hours, self.minutes, self.seconds
        ))
    }
}

#[cfg(test)]
mod time_parse_hhhmmss_pattern {
    use super::*;

    #[test]
    fn disallow_negative_time() {
        assert_eq!(
            Time::from_str("-1:23:45"),
            Err(SimpleError::new("Couldn't parse given time."))
        );
    }

    #[test]
    fn hours_allow_0_digit() {
        let given = Time::from_str(":01:23").unwrap();
        let expected = Time {
            hours: 0,
            minutes: 1,
            seconds: 23,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn hours_allow_0_digit_and_no_seconds() {
        let given = Time::from_str(":01").unwrap();
        let expected = Time {
            hours: 0,
            minutes: 1,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn hours_allow_1_digit() {
        let given = Time::from_str("1:23:45").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 45,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn hours_allow_1_digit_and_no_seconds() {
        let given = Time::from_str("1:23").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn hours_allow_2_digits() {
        let given = Time::from_str("10:23:45").unwrap();
        let expected = Time {
            hours: 10,
            minutes: 23,
            seconds: 45,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn hours_allow_2_digits_and_no_seconds() {
        let given = Time::from_str("10:23").unwrap();
        let expected = Time {
            hours: 10,
            minutes: 23,
            seconds: 00,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn hours_allow_3_digits() {
        let given = Time::from_str("100:23:45").unwrap();
        let expected = Time {
            hours: 100,
            minutes: 23,
            seconds: 45,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn hours_allow_3_digits_and_no_seconds() {
        let given = Time::from_str("100:23").unwrap();
        let expected = Time {
            hours: 100,
            minutes: 23,
            seconds: 00,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn minutes_allow_0_digits() {
        let given = Time::from_str("1::45").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 0,
            seconds: 45,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn minutes_allow_0_digits_and_no_seconds() {
        let given = Time::from_str("1:").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn minutes_allow_1_digit() {
        let given = Time::from_str("1:2:45").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 2,
            seconds: 45,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn minutes_allow_1_digit_and_no_seconds() {
        let given = Time::from_str("1:2").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 2,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn minutes_allow_2_digits() {
        let given = Time::from_str("1:23:45").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 45,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn minutes_allow_2_digits_and_no_seconds() {
        let given = Time::from_str("1:23").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn minutes_allow_up_to_59() {
        let given = Time::from_str("1:59:45").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 59,
            seconds: 45,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn minutes_normalize_more_than_59() {
        let given = Time::from_str("0:60:00").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 00,
            seconds: 00,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn minutes_allow_more_than_2_digits() {
        let given = Time::from_str("0:100:0").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 40,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn seconds_allow_0_digits() {
        let given = Time::from_str("1:23:").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 00,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn seconds_allow_1_digit() {
        let given = Time::from_str("1:23:4").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 4,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn seconds_allow_2_digits() {
        let given = Time::from_str("1:23:45").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 45,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn seconds_allow_more_than_2_digits() {
        let given = Time::from_str("0:00:3600").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn seconds_allow_up_to_59() {
        let given = Time::from_str("1:23:59").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 23,
            seconds: 59,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn seconds_normalize_more_than_59() {
        let given = Time::from_str("0:00:60").unwrap();
        let expected = Time {
            hours: 0,
            minutes: 1,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn normalize_both_minutes_and_seconds() {
        let given = Time::from_str("0:59:60").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }
}

#[cfg(test)]
mod time_parse_hhhdddd_pattern {
    use super::*;

    #[test]
    fn disallow_negative_time() {
        let given = Time::from_str("-1:23:45");
        let expected = Err(SimpleError::new("Couldn't parse given time."));
        assert_eq!(given, expected);
    }

    #[test]
    fn allow_comma_separator() {
        let given = Time::from_str("1,055").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 3,
            seconds: 18,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn allow_dot_separator() {
        let given = Time::from_str("1.055").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 3,
            seconds: 18,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn allow_no_decimal_part() {
        let given = Time::from_str("1").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn allow_no_decimal_part_after_separator() {
        let given = Time::from_str("1.").unwrap();
        let expected = Time {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn hours_allow_0_digits() {
        let given = Time::from_str(".055").unwrap();
        let expected = Time {
            hours: 0,
            minutes: 3,
            seconds: 18,
        };
        assert_eq!(given, expected)
    }
}

#[cfg(test)]
mod time_convert_from_decimal {

    use super::*;

    #[test]
    fn decimal_to_time() {
        let given = Time::from_decimal(1.055);
        let expected = Time {
            hours: 1,
            minutes: 3,
            seconds: 18,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn decimal_to_time_sixty_seconds_is_one_minute() {
        let sixty_seconds = 60.0 / 3600.0;
        let given = Time::from_decimal(sixty_seconds);
        let expected = Time {
            hours: 0,
            minutes: 1,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }

    #[test]
    fn decimal_to_time_normalized_sixty_minutes_is_one_hour() {
        let sixty_minutes = 60.0 / 60.0;
        let given = Time::from_decimal(sixty_minutes);
        let expected = Time {
            hours: 1,
            minutes: 0,
            seconds: 0,
        };
        assert_eq!(given, expected)
    }
}

#[cfg(test)]
mod time_convert_to_decimal {
    use super::*;

    #[test]
    fn time_to_decimal() {
        let given_time = Time {
            hours: 1,
            minutes: 30,
            seconds: 45,
        };
        assert_eq!(given_time.to_decimal(), 1.5125)
    }

    #[test]
    fn test_too_many_minutes() {
        let given_time = Time {
            hours: 1,
            minutes: 60,
            seconds: 00,
        };
        assert_eq!(given_time.to_decimal(), 2.0)
    }
}

#[cfg(test)]
mod time_formatting {
    use super::*;

    #[test]
    fn string_representation() {
        let given_time = Time {
            hours: 123,
            minutes: 2,
            seconds: 3,
        };
        let given = format!("{}", given_time);
        assert_eq!(given, "123:02:03");
    }
}

#[cfg(test)]
mod time_normalization {
    use super::*;

    #[test]
    fn preserves_already_normalized_values() {
        let given = Time::normalize(1, 59, 59);
        let expected = Time {
            hours: 1,
            minutes: 59,
            seconds: 59,
        };
        assert_eq!(given, expected);
    }

    #[test]
    fn normalizes_seconds() {
        let given = Time::normalize(1, 2, 60);
        let expected = Time {
            hours: 1,
            minutes: 3,
            seconds: 0,
        };
        assert_eq!(given, expected);
    }

    #[test]
    fn normalizes_minutes() {
        let given = Time::normalize(1, 60, 1);
        let expected = Time {
            hours: 2,
            minutes: 0,
            seconds: 1,
        };
        assert_eq!(given, expected);
    }

    #[test]
    fn normalizes_minutes_and_seconds() {
        let given = Time::normalize(1, 59, 60);
        let expected = Time {
            hours: 2,
            minutes: 0,
            seconds: 0,
        };
        assert_eq!(given, expected);
    }

    #[test]
    fn normalize_seconds_and_minutes_without_overflow() {
        let given = Time::normalize(64_425u16, 65_535u16, 65_535u16);
        let expected = Time {
            hours: 65_535u16,
            minutes: 27u8,
            seconds: 15u8,
        };
        assert_eq!(given, expected);
    }

    #[test]
    fn normalize_to_max_value() {
        let given = Time::normalize(64_426u16, 65_508u16, 65_519u16);
        let expected = Time {
            hours: 65_535u16,
            minutes: 59u8,
            seconds: 59u8,
        };
        assert_eq!(given, expected);
    }

    #[test]
    fn normalize_overflowing_resets_to_zero() {
        let given = Time::normalize(64_426u16, 65_508u16, 65_520u16);
        let expected = Time {
            hours: 0u16,
            minutes: 0u8,
            seconds: 0u8,
        };
        assert_eq!(given, expected);
    }
}
