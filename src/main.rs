extern crate vobject;
extern crate chrono;

use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use vobject::icalendar::ICalendar;
use chrono::naive::NaiveDate;
use chrono::prelude::*;
use chrono::Duration;
use chrono::Weekday;

fn main() {
    let mut buf = BufReader::new(File::open("./tests/ressources/ical_event.ics")
        .unwrap());
    let mut file_content = String::new();
    buf.read_to_string(&mut file_content); 

    let cal = ICalendar::build(&file_content).ok().unwrap();
    let now = Utc::now();
    let time_since_first = Duration::days((now.day0()) as i64);
    let seconds_of_first_day = now - time_since_first;
    let one_day = Duration::days(1);

    print!("{:>11} {:<8}\n",
        now.format("%B"),
        now.format("%Y")
    );
    println!("Su Mo Tu We Th Fr Sa");

    let this_month = now.month0() as i32;
    let mut current_day = seconds_of_first_day.naive_utc();

    if current_day.weekday().num_days_from_monday() > Weekday::Mon.num_days_from_monday() {
        for _ in 0..current_day.weekday().num_days_from_monday() {
            print!("   ");
        }
    }
    while current_day.month0() == this_month as u32 {
        print!(" {:>2} ", current_day.day());
        for event in cal.events() {
            match event {
                Ok(n)  => {
                    let event_date = n.dtstart().unwrap(); 
                    let from_string = NaiveDate::parse_from_str(event_date.raw(), "%Y%m%dT%H%M%SZ"); 
                    let from_calc = current_day.date();
                    if from_string == Ok(from_calc) {
                        println!("event is {:?} at {:?}:{:?}", n.summary().unwrap(), n.dtstart().unwrap(), n.dtend().unwrap())
                    }
                },
                Err(e) => println!("Error: {:?}", e),
            }
        }
        if current_day.weekday() as i32 == 6 {
            println!(""); // newline
        }
        current_day = current_day + one_day;
    }
}
