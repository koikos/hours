use std::fmt;

use regex::Regex;
use simple_error::SimpleError;


#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Time {
    hours: u16,
    minutes: u8,
    seconds: u8,
}

//todo: remove re pattern duplications in parser choser function, i.e. ::from(), and parsing
impl Time {
    pub fn from_str(time: &str) -> Result<Time, SimpleError> {
        return Time::from(&String::from(time));
    }

    pub fn from(time: &String) -> Result<Time, SimpleError> {
        let re_hhhmmss = Regex::new(r"^(?P<h>\d*):(?P<m>[0-5]?[0-9]?):(?P<s>[0-5]?[0-9]?)$").unwrap();
        let re_mmmss = Regex::new(r"^(?P<h>\d*):(?P<m>[0-5]?[0-9]?)$").unwrap();
        let re_hhhdddd = Regex::new(r"^\d*[,.]?\d*$").unwrap();

        return if re_hhhmmss.is_match(time) {
            Ok(Time::parse_hhhmmss(&time))
        } else if re_mmmss.is_match(time) {
            Ok(Time::parse_mmmss(&time))
        } else if re_hhhdddd.is_match(time) {
            Ok(Time::parse_hhhdddd(&time))
        } else {
            Err(SimpleError::new("Couldn't parse given time."))
        };
    }

    pub fn from_decimal(time: f32) -> Time {
        //todo: is there a standard way to convert to f32? should it be named :from_f32 / :from_f64?
        let hours = f32::floor(time);
        let mut remainder = time - hours;
        let minutes = f32::floor((remainder) * 60.0);
        remainder = remainder - minutes / 60.0;
        let seconds = f32::floor(remainder * 3600.0);
        return Time { hours: hours as u16, minutes: minutes as u8, seconds: seconds as u8 };
        //todo: is it safe conversions to u16, u8?? Error handling needed?
    }

    pub fn to_decimal(&self) -> f32 {
        let hours = f32::from(self.hours);
        let minutes = f32::from(self.minutes);
        let seconds = f32::from(self.seconds);
        return hours + minutes / 60.0 + seconds / 3600.0;
    }

    fn parse_hhhmmss(time: &String) -> Time {
        let re_hhhmmss = Regex::new(r"^(?P<h>\d*):(?P<m>[0-5]?[0-9]?):(?P<s>[0-5]?[0-9]?)$").unwrap();
        let caps = re_hhhmmss.captures(&time).unwrap();
        let hours = &caps["h"].parse::<u16>().unwrap();
        let minutes = &caps["m"].parse::<u8>().unwrap(); // todo: if "" then it should be 0
        let seconds = &caps["s"].parse::<u8>().unwrap(); // todo: if "" then it should be 0
        return Time { hours: *hours, minutes: *minutes, seconds: *seconds };
        //todo: error handling (IntParseError), return result?
    }

    fn parse_mmmss(time: &String) -> Time {
        let re_mmmss = Regex::new(r"^(?P<m>\d*):(?P<s>[0-5]?[0-9]?)$").unwrap();
        let caps = re_mmmss.captures(&time).unwrap();
        let minutes = &caps["m"].parse::<u16>().unwrap(); // todo: if "" then it should be 0
        let seconds = &caps["s"].parse::<u16>().unwrap(); // todo: if "" then it should be 0
        return Time::normalize(0, *minutes, *seconds);
        //todo: error handling (IntParseError)
        //todo: standarize minutes
    }

    fn parse_hhhdddd(time: &String) -> Time {
        let time = time.parse::<f32>().unwrap();
        return Time::from_decimal(time);
    }

    fn normalize(mut hours: u16, mut minutes: u16, mut seconds: u16) -> Time {
        minutes = minutes + u16::div_euclid(seconds, 60);
        seconds = u16::rem_euclid(seconds, 60);

        hours = hours + u16::div_euclid(minutes, 60);
        minutes = u16::rem_euclid(minutes, 60);

        return Time { hours: hours, minutes: minutes as u8, seconds: seconds as u8 };
        //todo: tests for standarize(), e.g. overflowing u16
    }
}

impl fmt::Display for Time {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(formatter, "{}:{}:{}", self.hours, self.minutes, self.seconds)
        formatter.write_fmt(format_args!("{}:{:0>2}:{:0>2}", self.hours, self.minutes, self.seconds))
    }
}

#[cfg(test)]
mod time_parse_hhhmmss_pattern {
    use super::*;

    #[test]
    fn disallow_negative_time() {
        assert_eq!(Time::from_str("-1:23:45"), Err(SimpleError::new("Couldn't parse given time.")));
    }

    #[test]
    fn hours_allow_1_digit() {
        assert_eq!(Time::from_str("1:23:45"), Ok(Time { hours: 1, minutes: 23, seconds: 45 }))
    }

    #[test]
    fn hours_allow_2_digits() {
        assert_eq!(Time::from_str("10:23:45"), Ok(Time { hours: 10, minutes: 23, seconds: 45 }))
    }

    #[test]
    fn hours_allow_3_digits() {
        assert_eq!(Time::from_str("100:23:45"), Ok(Time { hours: 100, minutes: 23, seconds: 45 }))
    }

    #[test]
    fn minutes_allow_0_digits() {
        assert_eq!(Time::from_str("1::45"), Ok(Time { hours: 1, minutes: 0, seconds: 45 }))
    }

    #[test]
    fn minutes_allow_1_digit() {
        assert_eq!(Time::from_str("1:2:45"), Ok(Time { hours: 1, minutes: 2, seconds: 45 }))
    }

    #[test]
    fn minutes_allow_2_digits() {
        assert_eq!(Time::from_str("1:23:45"), Ok(Time { hours: 1, minutes: 23, seconds: 45 }))
    }

    #[test]
    fn minutes_allow_up_to_59() {
        assert_eq!(Time::from_str("1:59:45"), Ok(Time { hours: 1, minutes: 59, seconds: 45 }))
    }

    #[test]
    fn minutes_disallow_more_than_59() {
        assert_eq!(Time::from_str("0:60:00"), Err(SimpleError::new("Couldn't parse given time.")))
    }

    #[test]
    fn seconds_allow_0_digits() {
        assert_eq!(Time::from_str("1:23:"), Ok(Time { hours: 1, minutes: 23, seconds: 00 }))
    }

    #[test]
    fn seconds_allow_1_digit() {
        assert_eq!(Time::from_str("1:23:4"), Ok(Time { hours: 1, minutes: 23, seconds: 4 }))
    }

    #[test]
    fn seconds_allow_2_digits() {
        assert_eq!(Time::from_str("1:23:45"), Ok(Time { hours: 1, minutes: 23, seconds: 45 }))
    }

    #[test]
    fn seconds_allow_up_to_59() {
        assert_eq!(Time::from_str("1:23:59"), Ok(Time { hours: 1, minutes: 23, seconds: 59 }))
    }

    #[test]
    fn seconds_disallow_more_than_59() {
        assert_eq!(Time::from_str("1:23:60"), Err(SimpleError::new("Couldn't parse given time.")))
    }
}

#[cfg(test)]
mod time_parse_mmmss_pattern {
    use super::*;

    #[test]
    fn disallow_negative_time() {
        assert_eq!(Time::from_str("-1:23"), Err(SimpleError::new("Couldn't parse given time.")));
    }

    #[test]
    fn minutes_allow_0_digit() {
        assert_eq!(Time::from_str(":01"), Ok(Time { hours: 0, minutes: 0, seconds: 1 }))
    }

    #[test]
    fn minutes_allow_1_digit() {
        assert_eq!(Time::from_str("1:23"), Ok(Time { hours: 0, minutes: 1, seconds: 23 }))
    }

    #[test]
    fn minutes_allow_2_digits() {
        assert_eq!(Time::from_str("10:23"), Ok(Time { hours: 0, minutes: 10, seconds: 23 }))
    }

    #[test]
    fn minutes_allow_3_digits() {
        assert_eq!(Time::from_str("100:23"), Ok(Time { hours: 1, minutes: 40, seconds: 23 }))
    }

    #[test]
    fn minutes_wrap() {
        assert_eq!(Time::from_str("60:01"), Ok(Time { hours: 1, minutes: 00, seconds: 01 }))
    }

    #[test]
    fn seconds_allow_0_digits() {
        assert_eq!(Time::from_str("1:"), Ok(Time { hours: 0, minutes: 1, seconds: 00 }))
    }

    #[test]
    fn seconds_allow_1_digit() {
        assert_eq!(Time::from_str("1:2"), Ok(Time { hours: 0, minutes: 1, seconds: 02 }))
    }

    #[test]
    fn seconds_allow_2_digits() {
        assert_eq!(Time::from_str("1:23"), Ok(Time { hours: 0, minutes: 1, seconds: 23 }))
    }

    #[test]
    fn seconds_allow_up_to_59() {
        assert_eq!(Time::from_str("0:59"), Ok(Time { hours: 0, minutes: 0, seconds: 59 }))
    }

    #[test]
    fn seconds_disallow_more_than_59() {
        assert_eq!(Time::from_str("0:60"), Err(SimpleError::new("Couldn't parse given time.")));
    }
}

#[cfg(test)]
mod time_parse_hhhdddd_pattern {
    use super::*;

    #[test]
    fn disallow_negative_time() {
        assert_eq!(Time::from_str("-1:23:45"), Err(SimpleError::new("Couldn't parse given time.")));
    }

    #[test]
    fn allow_comma_separator() {
        assert_eq!(Time::from_str("1,055"), Ok(Time { hours: 1, minutes: 3, seconds: 18 }))
    }

    #[test]
    fn allow_dot_separator() {
        assert_eq!(Time::from_str("1.055"), Ok(Time { hours: 1, minutes: 3, seconds: 18 }))
    }

    #[test]
    fn allow_none_separator_and_decimal_part() {
        assert_eq!(Time::from_str("1"), Ok(Time { hours: 1, minutes: 0, seconds: 0 }))
    }

    #[test]
    fn allow_separatore_and_none_decimal_part() {
        assert_eq!(Time::from_str("1."), Ok(Time { hours: 1, minutes: 0, seconds: 0 }))
    }

    #[test]
    fn hours_allow_0_digits() {
        assert_eq!(Time::from_str(".055"), Ok(Time { hours: 0, minutes: 3, seconds: 18 }))
    }
}

#[cfg(test)]
mod time_convert_from_decimal {
    use super::*;

    #[test]
    fn decimal_to_time() {
        assert_eq!(Time::from_decimal(1.055_f32), Time { hours: 1, minutes: 3, seconds: 18 })
    }

    //todo: here is crucial part, the conversion, many tests needed!!!
}

#[cfg(test)]
mod time_convert_to_decimal {
    use super::*;

    #[test]
    fn time_to_decimal() {
        assert_eq!(Time { hours: 1, minutes: 30, seconds: 45 }.to_decimal(), 1.5125)
    }

    #[test]
    fn test_too_many_minutes() {
        assert_eq!(Time { hours: 1, minutes: 60, seconds: 00 }.to_decimal(), 2.0)
    }

    //todo: here is crucial part, the conversion, many tests needed!!!
}

#[cfg(test)]
mod time_formatting {
    use super::*;

    #[test]
    fn string_representation() {
        assert_eq!(format!("{}", Time { hours: 123, minutes: 2, seconds: 3 }), "123:02:03");
    }
}

#[cfg(test)]
mod time_normalization {
    use super::*;

    #[test]
    fn preserves_already_normalized_values() {
        assert_eq!(Time::normalize(1, 59, 59), Time { hours: 1, minutes: 59, seconds: 59 });
    }

    #[test]
    fn normalizes_seconds() {
        assert_eq!(Time::normalize(1, 2, 60), Time { hours: 1, minutes: 3, seconds: 0 });
    }

    #[test]
    fn normalizes_minutes() {
        assert_eq!(Time::normalize(1, 60, 1), Time { hours: 2, minutes: 0, seconds: 1 });
    }

    #[test]
    fn normalizes_minutes_and_seconds() {
        assert_eq!(Time::normalize(1, 59, 60), Time { hours: 2, minutes: 0, seconds: 0 });
    }
}

