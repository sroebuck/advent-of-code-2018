#[macro_use]
extern crate lazy_static;

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::Timelike;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut records: Vec<Record> = reader.lines().map(|e| Record::parse(&e.unwrap())).collect();
    records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    // Collect record of frequency of sleeps per guard per minute after midnight...
    let mut guard_id = 0;
    let mut last_sleep_timestamp =
        NaiveDateTime::new(NaiveDate::from_ymd(0, 1, 1), NaiveTime::from_hms(0, 0, 0));
    let mut sleep_record: HashMap<usize, Vec<usize>> = HashMap::new();
    for r in records {
        match r.event {
            Event::Guard(x) => guard_id = x,
            Event::FallAsleep => last_sleep_timestamp = r.timestamp,
            Event::WakeUp => {
                let e = sleep_record.entry(guard_id).or_insert_with(|| vec![0; 60]);
                let start_min = last_sleep_timestamp.time().minute() as usize;
                let end_min = r.timestamp.time().minute() as usize;
                for z in e.iter_mut().take(end_min).skip(start_min) {
                    *z += 1;
                }
            }
        }
    }

    // Find guard with most sleep...
    let mut guard_id = 0;
    let mut max = 0;
    let mut record = vec![];
    for (id, rec) in sleep_record {
        let sum_of_sleeps: usize = rec.iter().sum();
        if sum_of_sleeps > max {
            guard_id = id;
            max = sum_of_sleeps;
            record = rec;
        }
    }

    // Find their most frequent sleep point...
    let mut local_max = 0;
    let mut minute = 0;
    for (m, &r) in record.iter().enumerate() {
        if r > local_max {
            local_max = r;
            minute = m;
        }
    }

    let mult = guard_id * minute;
    println!(
        "Guard #{guard_id} at {minute} mins after midnight.  {guard_id} * {minute} = {mult}",
        guard_id = guard_id,
        minute = minute,
        mult = mult
    );

    Ok(())
}

#[derive(Debug)]
enum Event {
    Guard(usize),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Record {
    timestamp: NaiveDateTime,
    event: Event,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^\[([^\]]+)\] ([^\n]+)$").unwrap();
}

impl Record {
    fn parse(input: &str) -> Record {
        let caps = RE.captures(input).unwrap();
        let date_time_string = caps.get(1).unwrap().as_str();
        let event_string = caps.get(2).unwrap().as_str();
        let date_time: NaiveDateTime =
            NaiveDateTime::parse_from_str(date_time_string, "%Y-%m-%d %H:%M").unwrap();
        let event = match event_string {
            "wakes up" => Event::WakeUp,
            "falls asleep" => Event::FallAsleep,
            x => {
                let l = x.len();
                let id: usize = x[7..l - 13].parse().unwrap();
                Event::Guard(id)
            }
        };
        Record {
            timestamp: date_time,
            event,
        }
    }
}
