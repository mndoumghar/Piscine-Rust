use chrono::{NaiveDate, Weekday, Duration};
 use chrono::Datelike;
pub fn middle_day(year: u32) -> Option<Weekday> {
    let dd = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

    let d = if dd { 366 } else { 365 };

    if d % 2 == 0 {
        return None;
    }
    
    println!("{}", d);

    let f = NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap();
    println!("{}", f);
    let middle = f + Duration::days((d / 2) as i64);
    println!("{}", middle);

    Some(middle.weekday())
}
