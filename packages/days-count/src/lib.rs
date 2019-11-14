use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Count the number of calendar days between two dates given as
/// timestamps in milliseconds. Make the assumption that One day is
/// 86_400_000 milliseconds (leap seconds are ignored)
pub fn count_days_between(timestamp_ms_a: u64, timestamp_ms_b: u64) -> u64 {
    let days_count_a = timestamp_ms_a / 1000 / 3600 / 24;
    let days_count_b = timestamp_ms_b / 1000 / 3600 / 24;
    let days_count_between = match days_count_a.checked_sub(days_count_b) {
        Some(difference) => difference,
        None => days_count_b - days_count_a,
    };

    return days_count_between;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_365_between_xmas_2018_and_xmas_2019() {
        assert_eq!(count_days_between(1545696000000, 1577232000000), 365);
    }

    #[test]
    fn it_returns_365_between_xmas_2019_and_xmas_2018() {
        assert_eq!(count_days_between(1577232000000, 1545696000000), 365);
    }

    #[test]
    fn it_returns_0_for_two_timestamps_in_the_same_day() {
        // 2019/11/14 at 00:00, and later in the day
        assert_eq!(count_days_between(1573689600000, 1573750188321), 0);
    }

    #[test]
    fn it_returns_29_in_february_of_a_leap_year() {
        // 2012/03/01 & 2012/02/01
        assert_eq!(count_days_between(1330560000000, 1328054400000), 29);
    }
}
