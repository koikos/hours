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
mod conversion_to_decimal {
    use super::*;

    //todo: it's a integration test... could Time be mocked and unit test could test just formatting
    #[test]
    fn prints_4_decimal_digits() -> Result<(), SimpleError> {
        let given = convert_time_to_decimal(&String::from("1:15:00"))?;
        let expected = String::from("1.2500");
        assert_eq!(given, expected);
        Ok(())
    }

    #[test]
    fn based_bug_parsing_minutes_overflow_while_no_seconds_given() {
        let current = convert_time_to_decimal(&String::from("1:66"));
        assert_eq!(current, Err(SimpleError::new("Couldn't parse given time.")));
    }
}
