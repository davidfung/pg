use chrono::{prelude::*, Duration};
use docx_rs::*;
use std::fs::{self, File};
use std::iter::from_fn;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        usage();
        std::process::exit(0);
    }

    match args[1].as_str() {
        "gen_bulletin_info" => {
            println!("Generating bulletin info...");
            let year = args[2].parse().unwrap();
            gen_bulletin_info(year);
        }
        _ => usage(),
    }
}

fn usage() {
    println!("Print version and usage"); //TODO
}

fn gen_bulletin_info(year: i32) {
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
fn test_wordx() {
    let name = "./hello.docx";
    let path = std::path::Path::new(name);
    let file = std::fs::File::create(&path).unwrap();

    // generate a Word document
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
        .build()
        .pack(file)
        .unwrap();

    // delete the generated Word document
    if std::path::Path::new(name).exists() {
        fs::remove_file(name).expect("unable to remove test_wordx() test file");
    }
}
