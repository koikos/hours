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
        //todo: better is to have two patterns hhhmmss and hhhmm - otherwise 1:66 is matched as 1:6:6
        //todo: or maybe lets allow overflowing (i.e. 66 minutes, and just standarize?
        let re_hhhmmss =
            Regex::new(r"^(?P<h>\d*):(?P<m>[0-5]?[0-9]?):?(?P<s>[0-5]?[0-9]?)$").unwrap();
        let re_hhhdddd = Regex::new(r"^\d*[,.]?\d*$").unwrap();

        return if re_hhhmmss.is_match(time) {
            let caps = re_hhhmmss.captures(&time).unwrap();
            Ok(Time::parse_hhhmmss(&caps["h"], &caps["m"], &caps["s"]))
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
        Time {
            hours: hours.parse::<u16>().unwrap_or(0_u16),
            minutes: minutes.parse::<u8>().unwrap_or(0_u8),
            seconds: seconds.parse::<u8>().unwrap_or(0_u8),
        }
    }

    fn parse_hhhdddd(time: &String) -> Time {
        let fixed_time = Time::fix_comma(&time);
        let time_f64 = fixed_time.parse::<Decimal>().unwrap();
        return Time::from_decimal(time_f64);

        //Todo: Maybe more readable would be: decimal_string.fix_comma.parse_as_f64.convert_to_time
    }

    fn normalize(mut hours: u16, mut minutes: u16, mut seconds: u16) -> Time {
        minutes = minutes + u16::div_euclid(seconds, 60);
        seconds = u16::rem_euclid(seconds, 60);

        hours = hours + u16::div_euclid(minutes, 60);
        minutes = u16::rem_euclid(minutes, 60);

        return Time {
            hours: hours,
            minutes: minutes as u8,
            seconds: seconds as u8,
        };
        //todo: tests for standarize(), e.g. overflowing u16
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
        //write!(formatter, "{}:{}:{}", self.hours, self.minutes, self.seconds)
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
    fn minutes_disallow_more_than_59() {
        assert_eq!(
            Time::from_str("0:60:00"),
            Err(SimpleError::new("Couldn't parse given time."))
        )
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
    fn seconds_disallow_more_than_59() {
        let given = Time::from_str("1:23:60");
        let expected = Err(SimpleError::new("Couldn't parse given time."));
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
    //todo: here is crucial part, the conversion, many tests needed!!!

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
}

#[cfg(test)]
mod time_convert_to_decimal {
    //todo: here is crucial part, the conversion, many tests needed!!!
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
}
