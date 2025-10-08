// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_log
// src/timestamp.rs

pub fn unix_time_to_human_readable(seconds: u64) -> String {
    let days_of_month = [31u64, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut days_till_now = seconds / 86400;
    let extra_time = seconds % 86400;
    let mut curr_year = 1970u64;

    loop {
        let is_leap = curr_year % 400 == 0 || (curr_year % 4 == 0 && curr_year % 100 != 0);
        let days_in_year = if is_leap { 366 } else { 365 };
        if days_till_now < days_in_year {
            break;
        }
        days_till_now -= days_in_year;
        curr_year += 1;
    }

    let mut extra_days = days_till_now + 1;
    let is_leap = curr_year % 400 == 0 || (curr_year % 4 == 0 && curr_year % 100 != 0);
    let mut month = 0u64;
    let mut index = 0usize;

    loop {
        let mut days = days_of_month[index];
        if index == 1 && is_leap {
            days = 29;
        }
        if extra_days < days {
            break;
        }
        month += 1;
        extra_days -= days;
        index += 1;
        if index > 11 {
            break;
        }
    }

    let date = if extra_days > 0 {
        month += 1;
        extra_days
    } else {
        if month == 2 && is_leap {
            29
        } else {
            days_of_month[(month - 1) as usize]
        }
    };

    let hours = extra_time / 3600;
    let minutes = (extra_time % 3600) / 60;
    let secs = (extra_time % 3600) % 60;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        curr_year, month, date, hours, minutes, secs
    )
}
