use chrono::Local;
use prettytable::{Cell, Row, Table};
use std::{collections::HashMap, env, error::Error};

mod api;
// use crate::api::{ROOM_1_GUID, ROOM_2_GUID};
use api::Client;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the user and password from environment variables
    let url = match env::var("BOOKING_URL") {
        Ok(value) => value,
        Err(_) => return Err("BOOKING_URL environment variable not set".into()),
    };
    let user = match env::var("BOOKING_USER") {
        Ok(value) => value,
        Err(_) => return Err("BOOKING_USER environment variable not set".into()),
    };

    let password = match env::var("BOOKING_PASSWORD") {
        Ok(value) => value,
        Err(_) => return Err("BOOKING_PASSWORD environment variable not set".into()),
    };

    let booking_rooms = match env::var("BOOKING_ROOMS") {
        Ok(value) => value,
        Err(_) => return Err("BOOKING_ROOMS environment variable not set".into()),
    };

    let booking_rooms_parsed = booking_rooms.split(",");

    let client = Client::new(url, user, password);

    let mut rooms_availability: Vec<HashMap<String, bool>> = Vec::new();

    for guid in booking_rooms_parsed {
        let availability = client.rooms_availability(guid, Local::now()).unwrap();
        let mut available: HashMap<String, bool> = Default::default();

        for (k, v) in availability.time_available.iter() {
            available.insert(k.to_string(), v.to_owned());
        }

        rooms_availability.push(available);
    }

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Time"),
        Cell::new("Room 1"),
        Cell::new("Room 2"),
    ]));

    let room_1 = sort_by_time(rooms_availability.get(0).unwrap());
    let room_2 = sort_by_time(rooms_availability.get(1).unwrap());

    let (day_start, day_end) = ("08:00", "17:00");

    let now = Local::now();

    let current_time = now.format("%H:%M").to_string();

    for (i, item) in room_1.into_iter().enumerate() {
        let time = item.0;

        if !time_in_range(time, day_start, day_end) {
            continue;
        }

        let is_current_range =
            time_in_range(current_time.as_str(), time, room_2.get(i + 1).unwrap().0);

        let (room_1_is_available, room_2_is_available) = (item.1, room_2.get(i).unwrap().1);

        table.add_row(Row::new(vec![
            Cell::new(
                (if is_current_range {
                    format!(">{}", time)
                } else {
                    time.to_string()
                })
                .as_str(),
            ),
            Cell::new(format!("{}", if room_1_is_available { "✅" } else { "❌" }).as_str()),
            Cell::new(format!("{}", if room_2_is_available { "✅" } else { "❌" }).as_str()),
        ]));
    }

    table.printstd();
    Ok(())
}

fn sort_by_time(map: &HashMap<String, bool>) -> Vec<(&str, bool)> {
    let mut times: Vec<&str> = map.keys().map(|k| k.as_str()).collect();
    times.sort_unstable_by(|&a, &b| time_cmp(a, b));
    times
        .into_iter()
        .map(|t| (t, *map.get(t).unwrap()))
        .collect()
}

fn time_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let (a_hours, a_minutes) = parse_time(a);
    let (b_hours, b_minutes) = parse_time(b);

    if a_hours == b_hours {
        a_minutes.cmp(&b_minutes)
    } else {
        a_hours.cmp(&b_hours)
    }
}

fn parse_time(t: &str) -> (u32, u32) {
    let parts: Vec<u32> = t.split(':').map(|s| s.parse().unwrap()).collect();

    (parts[0], parts[1])
}

fn time_in_range(t: &str, start: &str, end: &str) -> bool {
    let (t_hours, t_minutes) = parse_time(t);
    let (start_hours, start_minutes) = parse_time(start);
    let (end_hours, end_minutes) = parse_time(end);

    if t_hours < start_hours || t_hours > end_hours {
        return false;
    }

    if start_hours == end_hours && start_hours == t_hours {
        return (t_minutes >= start_minutes) && (t_minutes <= end_minutes);
    }

    if t_hours == start_hours {
        return t_minutes >= start_minutes;
    }

    if t_hours == end_hours {
        return t_minutes <= end_minutes;
    }

    true
}
