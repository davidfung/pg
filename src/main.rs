use chrono::{prelude::*, Duration};
use docx_rs::*;
use std::fs::{self, File};
use std::iter::from_fn;
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let year = 2023;
    let year_str = year.to_string();

    let path = Path::new(&year_str);
    if !path.exists() {
        fs::create_dir(path).unwrap();
    }

    let sundays = sundays_in_year(2023);
    for sunday in sundays.take(5) {
        let filename = format!("{} Bulletin Info.docx", sunday.format("%Y-%m-%d"));
        let path = Path::new(".").join(&year_str).join(&filename);
        let file = File::create(&path).unwrap();
        let body = format!("{} Bulletin Info", sunday.format("%Y-%m-%d"));

        Docx::new()
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text(body).size(36).bold()))
            .add_paragraph(Paragraph::new())
            .build()
            .pack(file)
            .unwrap();
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
