use std::num::ParseFloatError;

use regex::Regex;

pub fn parse_hhhmmss(time: &String) -> (u16, u8, u8) {
    let re_hhhmmss = Regex::new(r"^(?P<h>\d*):(?P<m>[0-5]?[0-9]?):(?P<s>[0-5]?[0-9]?)$").unwrap();
    let caps = re_hhhmmss.captures(&time).unwrap();
    let hours = &caps["h"].parse::<u16>().unwrap();
    let minutes = &caps["m"].parse::<u8>().unwrap(); // todo: if "" then it should be 0
    let seconds = &caps["s"].parse::<u8>().unwrap(); // todo: if "" then it should be 0
    return (*hours, *minutes, *seconds);
    //todo: error handling (IntParseError)
}

pub fn parse_mmmss(time: &String) -> (u16, u8) {
    let re_mmmss = Regex::new(r"^(?P<m>\d*):(?P<s>[0-5]?[0-9]?)$").unwrap();
    let caps = re_mmmss.captures(&time).unwrap();
    let minutes = &caps["m"].parse::<u16>().unwrap(); // todo: if "" then it should be 0
    let seconds = &caps["s"].parse::<u8>().unwrap(); // todo: if "" then it should be 0
    return (*minutes, *seconds);
    //todo: error handling (IntParseError)
}

//todo: error handling ParseFloatError
pub fn parse_hhhdddd(time: &String) -> Result<f32, ParseFloatError> {
    return time.parse::<f32>();
}

#[cfg(test)]
mod tests_hhhmmss_pattern {
    use super::*;

    #[test]
    fn hours_allow_1_digit() {
        assert_eq!(parse_hhhmmss(&String::from("1:00:00")), (1, 0, 0));
    }

    #[test]
    fn hours_allow_2_digits() {
        assert_eq!(parse_hhhmmss(&String::from("10:00:00")), (10, 0, 0));
    }

    #[test]
    fn hours_allow_3_digits() {
        assert_eq!(parse_hhhmmss(&String::from("100:00:00")), (100, 0, 0));
    }

    #[test]
    fn minutes_allow_0_digits() {
        assert_eq!(parse_hhhmmss(&String::from("0::00")), (0, 0, 0));
    }

    #[test]
    fn minutes_allow_1_digit() {
        assert_eq!(parse_hhhmmss(&String::from("0:1:00")), (0, 1, 0));
    }

    #[test]
    fn minutes_allow_2_digits() {
        assert_eq!(parse_hhhmmss(&String::from("0:11:00")), (0, 11, 0));
    }

    #[test]
    fn minutes_allow_up_to_59() {
        assert_eq!(parse_hhhmmss(&String::from("0:59:00")), (0, 59, 0));
    }

    #[test]
    fn seconds_allow_0_digits() {
        assert_eq!(parse_hhhmmss(&String::from("1:23:")), (1, 23, 0));
    }

    #[test]
    fn seconds_allow_1_digit() {
        assert_eq!(parse_hhhmmss(&String::from("00:00:1")), (0, 0, 1));
    }

    #[test]
    fn seconds_allow_2_digits() {
        assert_eq!(parse_hhhmmss(&String::from("0:00:01")), (0, 0, 1));
    }

    #[test]
    fn seconds_allow_up_to_59() {
        assert_eq!(parse_hhhmmss(&String::from("0:00:59")), (0, 0, 59));
    }
}

#[cfg(test)]
mod tests_mmmss_pattern {
    use super::*;

    #[test]
    fn minutes_allow_0_digit() {
        assert_eq!(parse_mmmss(&String::from(":01")), (0, 1));
    }

    #[test]
    fn minutes_allow_1_digit() {
        assert_eq!(parse_mmmss(&String::from("1:00")), (1, 0));
    }

    #[test]
    fn minutes_allow_2_digits() {
        assert_eq!(parse_mmmss(&String::from("10:00")), (10, 0));
    }

    #[test]
    fn minutes_allow_3_digits() {
        assert_eq!(parse_mmmss(&String::from("100:00")), (100, 0));
    }

    #[test]
    fn seconds_allow_0_digits() {
        assert_eq!(parse_mmmss(&String::from("1:")), (1, 0));
    }

    #[test]
    fn seconds_allow_1_digit() {
        assert_eq!(parse_mmmss(&String::from("0:1")), (0, 1));
    }

    #[test]
    fn seconds_allow_2_digits() {
        assert_eq!(parse_mmmss(&String::from("0:01")), (0, 1));
    }

    #[test]
    fn seconds_allow_up_to_59() {
        assert_eq!(parse_mmmss(&String::from("0:59")), (0, 59));
    }

    #[test]
    #[ignore]
    //todo: remove this test or fill it with implementation
    fn seconds_disallow_more_than_59() {}
}

#[cfg(test)]
mod tests_hhhdddd_parser {
    use super::*;

    #[test]
    fn allow_comma_separator() {
        assert_eq!(parse_hhhdddd(&String::from("0,1")), Ok(0.1));
    }

    #[test]
    fn allow_dot_separator() {
        assert_eq!(parse_hhhdddd(&String::from("0.1")), Ok(0.1));
    }

    #[test]
    fn allow_none_separator_and_decimal_part() {
        assert_eq!(parse_hhhdddd(&String::from("1")), Ok(1.0));
    }

    #[test]
    fn allow_separatore_and_none_decimal_part() {
        assert_eq!(parse_hhhdddd(&String::from("1.")), Ok(1.0));
    }

    #[test]
    fn hours_allow_0_digits() {
        assert_eq!(parse_hhhdddd(&String::from(".1000")), Ok(0.1));
    }
}