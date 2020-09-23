pub fn time_to_hours(hours: u16, minutes: u16, seconds: u16) -> f32 {
    let hours = f32::from(hours);
    let minutes = f32::from(minutes);
    let seconds = f32::from(seconds);
    return hours + minutes / 60.0 + seconds / 3600.0;
}

pub fn hours_to_time(time: f32) -> (u16, u8, u8) {
    let hours = f32::floor(time);
    let mut remainder = time - hours;
    let minutes = f32::floor((remainder) * 60.0);
    remainder = remainder - minutes / 60.0;
    let seconds = f32::floor(remainder * 3600.0);
    return (hours as u16, minutes as u8, seconds as u8);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // ********************************************************************* Tests for time_to_hours
    #[test]
    fn test_time_to_hours() {
        assert_eq!(time_to_hours(1, 30, 45), 1.5125);
    }

    #[test]
    fn test_too_many_minutes_and_seconds() {
        assert_eq!(time_to_hours(0, 60, 3600), 2.0);
    }

    // ********************************************************************* Tests for hours_to_time
    #[test]
    fn test_hours_to_time() {
        assert_eq!(hours_to_time(1.5125), (1, 30, 45));
    }
}
