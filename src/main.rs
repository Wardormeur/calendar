extern crate vobject;
extern crate time;
extern crate chrono;

use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use vobject::icalendar::ICalendar;
use chrono::naive::NaiveDate;

fn main() {
    let mut buf = BufReader::new(File::open("./tests/ressources/ical_event.ics")
        .unwrap());
    let mut file_content = String::new();
    buf.read_to_string(&mut file_content); 

    // text to components
    /*let cal_component = vobject::parse_component(&file_content).unwrap();
    let cal = vobject::icalendar:ICalendar.from_component(cal);
    println!("{}", cal.name);
    let event = &cal.subcomponents[0];*/

    // text to cal
    let cal = vobject::icalendar::ICalendar::build(&file_content).ok().unwrap();
    let now = time::now_utc();
    let time_since_first = time::Duration::days((now.tm_mday - 1) as i64);
    let seconds_of_first_day = now.to_timespec() - time_since_first;
    let first_of_the_month = time::at_utc(seconds_of_first_day);
    let one_day = time::Duration::hours(24);

    print!("{:>11} {:<8}\n",
        now.strftime("%B").ok().unwrap().to_string(),
        now.strftime("%Y").ok().unwrap().to_string()
    );
    println!("Su Mo Tu We Th Fr Sa");

    let this_month = now.tm_mon;
    let mut current_day = first_of_the_month;

    if current_day.tm_wday > 0 {
        for _ in 0..current_day.tm_wday {
            print!("   ");
        }
    }
    while current_day.tm_mon == this_month {
        print!(" {:>2} ", current_day.tm_mday);
        for event in cal.events() {
            match event {
                Ok(n)  => {
                    let event_date = n.dtstart().unwrap(); 
                    let from_string = NaiveDate::parse_from_str(event_date.raw(), "%Y%m%dT%H%M%SZ"); 
                    let from_cal = NaiveDate::from_ymd(1900 + current_day.tm_year, (current_day.tm_mon as u32) + 1, current_day.tm_mday as u32);
                    if (from_string == Ok(from_cal)) {
                        println!("event is {:?} at {:?}:{:?}", n.summary().unwrap(), n.dtstart().unwrap(), n.dtend().unwrap())
                    }
                },
                Err(e) => println!("Error: {:?}", e),
            }
        }
        if current_day.tm_wday == 6 {
            println!(""); // newline
        }
        current_day = current_day + one_day;
    }
}
