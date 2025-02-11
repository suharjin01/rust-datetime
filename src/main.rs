fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Duration, NaiveDate};


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
}
