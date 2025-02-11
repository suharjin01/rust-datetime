fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
    use chrono_tz::{Asia, Tz};


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

    // Time Zone
    // time zone menggunakan library tambahan
    #[test]
    fn test_time_zone() {
        let utc_date_time: NaiveDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 2, 11).unwrap(),
            NaiveTime::from_hms_opt(21, 47, 25).unwrap()
        );

        let asia_makassar = Asia::Makassar;
        let asia_makassar_time = asia_makassar.from_utc_datetime(&utc_date_time);

        println!("{}", utc_date_time);
        println!("{}", asia_makassar_time);
    }

    // Date Time dengan Time Zone
    #[test]
    fn test_date_time_with_time_zone() {
        let utc_date_time: DateTime<Utc> = Utc::now();
        let asia_makassar_date_time: DateTime<Tz> = Asia::Makassar.from_utc_datetime(&utc_date_time.naive_utc());

        println!("{}", utc_date_time);
        println!("{}", asia_makassar_date_time);

        let local_date_time = Local::now();
        let asia_makassar_date_time = Asia::Makassar.from_local_datetime(&local_date_time.naive_local()).unwrap();

        println!("{}", local_date_time);
        println!("{}", asia_makassar_date_time);
    }

    // Parsing
    #[test]
    fn test_parsing() {
        let string = String::from("2025-02-11 22:10:10 +0800");
        let time = DateTime::parse_from_str(&string, "%Y-%m-%d %H:%M:%S %z").unwrap();

        println!("{}", time.year());
        println!("{}", time.month());
        println!("{}", time.day());
        println!("{}", time.hour());
        println!("{}", time.minute());
        println!("{}", time.second());
        println!("{}", time.nanosecond());
        println!("{}", time.timezone());
    }

    // Format
    #[test]
    fn test_format() {
        let time = Local::now();
        let formatted = time.format("%Y-%m-%d %H:%M:%S %z").to_string();

        println!("{}", formatted);
    }
}
