use chrono::{prelude::*, Duration, LocalResult};
use std::iter::from_fn;

fn main() {
    println!("Hello, world!");
    let sundays = sundays_in_year(2022);
    for elm in sundays {
        println!("{:?}", elm.single().unwrap());
    }
}

fn sundays_in_year(year: i32) -> impl Iterator<Item = LocalResult<DateTime<Local>>> {
    let mut d = Local
        .with_ymd_and_hms(year, 1, 1, 0, 0, 0)
        .single()
        .unwrap();
    while d.weekday() != Weekday::Sun {
        d = d + Duration::days(1);
    }
    d = d - Duration::days(7);
    let sundays = from_fn(move || {
        d = d + Duration::days(7);
        if d.year() != year {
            None
        } else {
            Some(LocalResult::Single(d))
        }
    });

    sundays
}

#[test]
fn test1() {
    let mut count = 0;
    let counter = std::iter::from_fn(move || {
        // Increment our count. This is why we started at zero.
        count += 1;

        // Check to see if we've finished counting or not.
        if count < 6 {
            Some(count)
        } else {
            None
        }
    });
    assert_eq!(counter.collect::<Vec<_>>(), &[1, 2, 3, 4, 5]);
}

// fn first_sunday(year: u16) -> DateTime {}

#[test]
fn test_first_sunday() {}
