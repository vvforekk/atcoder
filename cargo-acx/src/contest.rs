use chrono::{DateTime, Duration, Local, TimeZone};
use itertools::izip;
use kuchiki::traits::TendrilSink;
use prettytable::{
    cell,
    format::{FormatBuilder, LinePosition, LineSeparator},
    row, Table,
};
use reqwest::Url;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Contest {
    pub name: String,
    pub id: String,
    pub start_time: DateTime<Local>,
    pub duration: Duration,
}

pub fn get_upcoming_contests() -> anyhow::Result<Vec<Contest>> {
    let html = reqwest::blocking::get("https://atcoder.jp/contests?lang=en")?.text()?;
    let document = kuchiki::parse_html().one(html);

    let start_time_selector = "
        #contest-table-upcoming >
        div:nth-child(2) >
        div:nth-child(1) >
        table:nth-child(1) >
        tbody:nth-child(2) >
        tr >
        td:nth-child(1) >
        a
    ";

    let contest_name_selector = "
        #contest-table-upcoming >
        div:nth-child(2) >
        div:nth-child(1) >
        table:nth-child(1) >
        tbody:nth-child(2) >
        tr >
        td:nth-child(2) >
        a
    ";

    let duration_selector = "
        #contest-table-upcoming >
        div:nth-child(2) >
        div:nth-child(1) >
        table:nth-child(1) >
        tbody:nth-child(2) >
        tr >
        td:nth-child(3)
    ";

    let rated_range_selector = "
        #contest-table-upcoming >
        div:nth-child(2) >
        div:nth-child(1) >
        table:nth-child(1) >
        tbody:nth-child(2) >
        tr >
        td:nth-child(4)
    ";

    Ok(izip!(
        document.select(start_time_selector).unwrap(),
        document.select(contest_name_selector).unwrap(),
        document.select(duration_selector).unwrap(),
        document.select(rated_range_selector).unwrap()
    )
    .map(
        |(start_time, contest_name, duration, _rated_range)| Contest {
            name: contest_name.text_contents(),
            id: contest_name
                .attributes
                .borrow()
                .get("href")
                .unwrap()
                .strip_prefix("/contests/")
                .unwrap()
                .to_string(),
            start_time: {
                let timeanddate_url =
                    Url::from_str(start_time.attributes.borrow().get("href").unwrap()).unwrap();
                let time_stamp = timeanddate_url
                    .query_pairs()
                    .find(|(k, _)| k == "iso")
                    .unwrap()
                    .1
                    .to_string();
                Local.datetime_from_str(&time_stamp, "%Y%m%dT%H%M").unwrap()
            },
            duration: {
                let text = duration.text_contents();
                let (hour, minute) = text.split_once(':').unwrap();
                Duration::hours(hour.parse().unwrap()) + Duration::minutes(minute.parse().unwrap())
            },
        },
    )
    .collect())
}

#[test]
fn get_upcoming_contests_test() {
    let upcoming_contests = get_upcoming_contests().unwrap();
    dbg!(upcoming_contests);
}

pub fn print_contests(contests: &[Contest]) -> anyhow::Result<()> {
    let mut table = Table::new();
    table.set_format(
        FormatBuilder::new()
            .padding(1, 1)
            .column_separator('│')
            .borders('│')
            .separator(LinePosition::Top, LineSeparator::new('─', '┬', '┌', '┐'))
            .separator(LinePosition::Title, LineSeparator::new('─', '┼', '├', '┤'))
            .separator(LinePosition::Intern, LineSeparator::new('─', '┼', '├', '┤'))
            .separator(LinePosition::Bottom, LineSeparator::new('─', '┴', '└', '┘'))
            .build(),
    );

    table.set_titles(row![
        b -> "Contest Name",
        b -> "ID",
        b -> format!("Start Time"),
        b -> "Duration"
    ]);

    for Contest {
        name,
        id,
        start_time,
        duration,
    } in contests
    {
        table.add_row(row![name, id, start_time, display_duration(duration)]);
    }

    table.print_tty(true);

    Ok(())
}

fn display_duration(value: &Duration) -> String {
    if value >= &Duration::days(1) {
        format!(
            "{}d {}h {}min",
            value.num_days(),
            value.num_hours() % 60,
            value.num_minutes() % 60
        )
    } else if value >= &Duration::hours(1) {
        format!("{}h {}min", value.num_hours(), value.num_minutes() % 60)
    } else {
        format!("{}min", value.num_minutes())
    }
}
