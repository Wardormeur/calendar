extern crate ical;
extern crate time;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let buf = BufReader::new(File::open("./tests/ressources/ical_input.ics")
        .unwrap());

    let reader = ical::IcalParser::new(buf);
    let mut events: Vec<ical::IcalEvents> = vec::new();
    for line in reader {
        let cal = line.ok().unwrap();
        println!("{:?}", cal.events);
    }
    /*reader.events.foreach(|event, iter| {
        println!("event: {}", event.properties);    
    });*/
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
        print!("{:>2} ", current_day.tm_mday);
        if current_day.tm_wday == 6 {
            println!(""); // newline
        }
        current_day = current_day + one_day;
    }
}
