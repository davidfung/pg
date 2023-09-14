use chrono::{prelude::*, Duration};
use docx_rs::*;
use std::iter::from_fn;

fn main() {
    println!("Hello, world!");
    let sundays = sundays_in_year(2023);
    for sunday in sundays {
        println!("{:?}", sunday);
    }
}

fn sundays_in_year(year: i32) -> impl Iterator<Item = DateTime<Local>> {
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
            Some(d)
        }
    });

    sundays
}

#[test]
fn test_sundays_in_year() {
    let mut sundays = sundays_in_year(2022);
    assert_eq!(sundays.count(), 52);
    sundays = sundays_in_year(2022);
    let x = sundays.next();
    assert_eq!(x, Local.with_ymd_and_hms(2022, 1, 2, 0, 0, 0).single());
}

#[test]
fn test_wordx() -> Result<(), DocxError> {
    let path = std::path::Path::new("./hello.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .build()
        .pack(file)?;
    Ok(())
}
