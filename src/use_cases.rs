use simple_error::SimpleError;

use crate::time;

// hhh:mm:ss -> hhh.dddd
pub fn convert_time_to_decimal(hhhmmss: &String) -> Result<String, SimpleError> {
    let time = time::Time::from(&hhhmmss)?;
    log::debug!("{:?}", time);
    Ok(format!("{:.4}", time.to_decimal()))
}

// hhh.dddd -> hhh:mm:ss
pub fn convert_decimal_to_time(hhhdddd: &String) -> Result<String, SimpleError> {
    let time = time::Time::from(&hhhdddd)?;
    log::debug!("{:?}", time);
    Ok(time.to_string())
}

#[cfg(test)]
mod acceptance_tests {
    use super::*;

    #[test]
    fn prints_4_decimal_digits() -> Result<(), SimpleError> {
        let given = convert_time_to_decimal(&String::from("1:15:00"))?;
        let expected = String::from("1.2500");
        assert_eq!(given, expected);
        Ok(())
    }

    #[test]
    fn based_bug_parsing_minutes_overflow_while_no_seconds_given() -> Result<(), SimpleError> {
        let time1 = convert_time_to_decimal(&String::from("2:06:00"))?;
        let time2 = convert_time_to_decimal(&String::from("1:66"))?;
        assert_eq!(time1, time2);
        Ok(())
    }
}
