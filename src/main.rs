fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike};


    #[test]
    fn test_date() {
        let date = NaiveDate::from_ymd_opt(2025, 2, 11).unwrap();

        println!("{}", date.year());
        println!("{}", date.month());
        println!("{}", date.day());
    }

    // Contoh Duration
    #[test]
    fn test_duration() {
        let date = NaiveDate::from_ymd_opt(2025, 2, 11).unwrap();
        let duration = Duration::days(10);
        let new_date: NaiveDate = date + duration;

        println!("{}", date);
        println!("{}", new_date);
    }

    // Time
    #[test]
    fn test_time() {
        let time = NaiveTime::from_hms_milli_opt(21, 17, 30, 500).unwrap();

        println!("{}", time.hour());
        println!("{}", time.minute());
        println!("{}", time.second());
        println!("{}", time.nanosecond());
    }

    // Date Time
    #[test]
    fn test_datetime() {
        let date_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 2, 11).unwrap(),
            NaiveTime::from_hms_opt(21, 40, 55).unwrap()
        );

        println!("{}", date_time.date().year());
        println!("{}", date_time.date().month());
        println!("{}", date_time.date().day());
        println!("{}", date_time.time().hour());
        println!("{}", date_time.time().minute());
        println!("{}", date_time.time().second());
        println!("{}", date_time.time().nanosecond());
    }

    // Time Zone
    // time zone secara manual
    #[test]
    fn test_offset() {
        let utc_date_time: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 2, 11).unwrap(),
            NaiveTime::from_hms_opt(21, 47, 25).unwrap()
        );

        let asia_makassar = FixedOffset::east_opt(8 * 3600).unwrap();
        let asia_makassar_time = asia_makassar.from_utc_datetime(&utc_date_time);

        println!("{}", utc_date_time);
        println!("{}", asia_makassar_time);
    }
}
