use chrono::{prelude::*, Duration};
use docx_rs::*;
use std::env::Args;
use std::fs::{self, File};
use std::iter::from_fn;
use std::path::Path;

fn main() {
    let mut args = std::env::args();

    if args.len() <= 1 {
        usage();
        std::process::exit(0);
    }

    args.next();
    let cmd = args.next().unwrap();
    match cmd.as_str() {
        "gen_bulletin_info" => {
            gen_bulletin_info_helper(args);
        }
        _ => usage(),
    }
}

fn usage() {
    let msg = "
pg - Utility app for PMPG (version 1.0b)

Commands:
   gen_bulletin_info      Generate bulletin info templates for a whole year

";
    println!("{msg}");
}

fn gen_bulletin_info_helper(mut args: Args) {
    if args.len() < 1 {
        println!("Usage: pg gen_bulletin_info year");
        std::process::exit(1);
    }
    print!("Generating bulletin info... ");
    let year: i32 = args.next().unwrap().parse().unwrap();
    gen_bulletin_info(year);
    println!("done");
}

fn gen_bulletin_info(year: i32) {
    let year_str = year.to_string();

    let path = Path::new(&year_str);
    if !path.exists() {
        fs::create_dir(path).unwrap();
    }

    let sundays = sundays_in_year(year);
    for sunday in sundays {
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
