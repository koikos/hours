use simple_error::SimpleError;

mod converters;

pub fn convert_time(time: &String) -> Result<String, SimpleError> {
    use regex::Regex;
    let re_hhhmmss = Regex::new(r"^\d*:[0-5]?[0-9]?:[0-5]?[0-9]?$").unwrap();
    let re_mmmss = Regex::new(r"^\d*:[0-5]?[0-9]?$").unwrap();
    let re_hhddddd = Regex::new(r"^\d*[,.]?\d*$").unwrap();

    let result;
    if re_hhhmmss.is_match(time) {
        result = String::from("HHH:MM:SS -> HHH.DDDDD");
    } else if re_mmmss.is_match(time) {
        result = String::from("MMM:SS -> HHH.DDDDD");
    } else if re_hhddddd.is_match(time) {
        result = String::from("HHH.DDDDD -> HHH:MM:SS");
    } else {
        return Err(SimpleError::new("Couldn't parse given time."));
    }
    Ok(result)
}

#[cfg(test)]
mod tests_hhhmmss_pattern {
    use super::*;

    #[test]
    fn disallow_negative_time() {
        assert_eq!(convert_time(&String::from("-1:00:00")), Err(SimpleError::new("Couldn't parse given time.")));
    }

    #[test]
    fn hours_allow_1_digit() {
        assert_eq!(convert_time(&String::from("1:00:00")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn hours_allow_2_digits() {
        assert_eq!(convert_time(&String::from("10:00:00")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn hours_allow_3_digits() {
        assert_eq!(convert_time(&String::from("100:00:00")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn minutes_allow_0_digits() {
        assert_eq!(convert_time(&String::from("0::00")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn minutes_allow_1_digit() {
        assert_eq!(convert_time(&String::from("0:0:00")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn minutes_allow_2_digits() {
        assert_eq!(convert_time(&String::from("0:00:00")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn minutes_allow_up_to_59() {
        assert_eq!(convert_time(&String::from("0:59:00")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn minutes_disallow_more_than_59() {
        assert_eq!(convert_time(&String::from("0:60:00")), Err(SimpleError::new("Couldn't parse given time.")));
    }

    #[test]
    fn seconds_allow_0_digits() {
        assert_eq!(convert_time(&String::from("1:00:")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn seconds_allow_1_digit() {
        assert_eq!(convert_time(&String::from("00:00:1")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn seconds_allow_2_digits() {
        assert_eq!(convert_time(&String::from("0:00:01")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn seconds_allow_up_to_59() {
        assert_eq!(convert_time(&String::from("0:00:59")), Ok(String::from("HHH:MM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn seconds_disallow_more_than_59() {
        assert_eq!(convert_time(&String::from("0:00:60")), Err(SimpleError::new("Couldn't parse given time.")));
    }
}

#[cfg(test)]
mod tests_mmmss_pattern {
    use super::*;

    #[test]
    fn disallow_negative_time() {
        assert_eq!(convert_time(&String::from("-1:00")), Err(SimpleError::new("Couldn't parse given time.")));
    }

    #[test]
    fn minutes_allow_0_digit() {
        assert_eq!(convert_time(&String::from(":01")), Ok(String::from("MMM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn minutes_allow_1_digit() {
        assert_eq!(convert_time(&String::from("1:00")), Ok(String::from("MMM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn minutes_allow_2_digits() {
        assert_eq!(convert_time(&String::from("10:00")), Ok(String::from("MMM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn minutes_allow_3_digits() {
        assert_eq!(convert_time(&String::from("100:00")), Ok(String::from("MMM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn seconds_allow_0_digits() {
        assert_eq!(convert_time(&String::from("1:")), Ok(String::from("MMM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn seconds_allow_1_digit() {
        assert_eq!(convert_time(&String::from("0:1")), Ok(String::from("MMM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn seconds_allow_2_digits() {
        assert_eq!(convert_time(&String::from("0:01")), Ok(String::from("MMM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn seconds_allow_up_to_59() {
        assert_eq!(convert_time(&String::from("0:59")), Ok(String::from("MMM:SS -> HHH.DDDDD")));
    }

    #[test]
    fn seconds_disallow_more_than_59() {
        assert_eq!(convert_time(&String::from("0:60")), Err(SimpleError::new("Couldn't parse given time.")));
    }
}

#[cfg(test)]
mod tests_hhhdddd_pattern {
    use super::*;

    #[test]
    fn disallow_negative_time() {
        assert_eq!(convert_time(&String::from("-1.0")), Err(SimpleError::new("Couldn't parse given time.")));
    }

    #[test]
    fn allow_comma_separator() {
        assert_eq!(convert_time(&String::from("0,1")), Ok(String::from("HHH.DDDDD -> HHH:MM:SS")));
    }

    #[test]
    fn allow_dot_separator() {
        assert_eq!(convert_time(&String::from("0.1")), Ok(String::from("HHH.DDDDD -> HHH:MM:SS")));
    }

    #[test]
    fn allow_none_separator_and_decimal_part() {
        assert_eq!(convert_time(&String::from("1")), Ok(String::from("HHH.DDDDD -> HHH:MM:SS")));
    }

    #[test]
    fn allow_separatore_and_none_decimal_part() {
        assert_eq!(convert_time(&String::from("1.")), Ok(String::from("HHH.DDDDD -> HHH:MM:SS")));
    }

    #[test]
    fn hours_allow_0_digits() {
        assert_eq!(convert_time(&String::from(".1000")), Ok(String::from("HHH.DDDDD -> HHH:MM:SS")));
    }
}
