use std::{fs::read_to_string, io::BufWriter};

use chrono::Datelike;
use ical_syntax::{
    self,
    structure::icalstream::components::{EventC, ICalObject, ICalStream},
    write::{
        Writer,
        icalstream::typed_writers::{EventWriterExt, ICalObjectWriterExt},
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();
    let stdout = stdout.lock();

    let generated = chrono::Utc::now();

    let mut ics = Writer::<_, ICalStream>::new(BufWriter::new(stdout));
    let mut ico = ics.component(ICalObject)?;

    ico.version("2.0")?;
    ico.prod_id("urn:tag:johanneshoff.com,birthical")?;

    for mut line in read_to_string("birthdays.txt").unwrap().lines() {
        if let Some(idx) = line.find('#') {
            line = &line[..idx];
        }
        if let Some((_header, body)) = line.split_at_checked(2)
            && let Some((date_str, name)) = body.split_at_checked(10)
        {
            let name = name.trim();
            if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                for year in 2025..2027 {
                    let mut ev = ico.component(EventC)?;
                    let age = year - date.year();
                    ev.uid(format!("{}-{}-{}", year, name, date))?;
                    ev.summary(format!("{} {} år", name, age))?;
                    ev.dtstamp(generated)?;
                    ev.time_transparency(
                        ical_syntax::write::value_types::TimeTransparency::Transparent,
                    )?;
                    ev.dtstart(date.with_year(year).unwrap())?;
                    ev.end()?;
                }
            } else {
                if let Ok(month) = date_str[5..7].parse::<u32>()
                    && let Ok(day) = date_str[8..10].parse::<u32>()
                    && let Some(cal_date) = chrono::NaiveDate::from_ymd_opt(2025, month, day)
                {
                    let mut ev = ico.component(EventC)?;
                    ev.uid(format!("{}-{}", name, date_str))?;
                    ev.summary(format!("{} bursdag", name))?;
                    ev.dtstamp(generated)?;
                    ev.time_transparency(
                        ical_syntax::write::value_types::TimeTransparency::Transparent,
                    )?;
                    // TODO add RRULE:FREQ=YEARLY
                    ev.dtstart(cal_date)?;
                    ev.end()?;
                }
            }
        }
    }

    ico.end()?;

    drop(ics);

    Ok(())
}
