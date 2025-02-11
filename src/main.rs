fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDate};


    #[test]
    fn test_date() {
        let date = NaiveDate::from_ymd_opt(2025, 2, 11).unwrap();

        println!("{}", date.year());
        println!("{}", date.month());
        println!("{}", date.day());
    }
}
