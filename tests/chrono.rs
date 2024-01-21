use chrono::{prelude::*, Datelike, Duration};
use wrapper_chrono::wrapper::{ChronoTrait, WrapperChrono};

#[test]
fn test_new_date_utc_now() {
    let date = WrapperChrono::new_date_utc_now();
    assert_eq!(date.timezone(), Utc);
}

#[test]
fn test_new_date_local_now() {
    let date = WrapperChrono::new_date_local_now();
    let now = Local::now();
    assert_eq!(date.date_naive(), now.date_naive());
}

#[test]

fn test_new_date_utc_add_time_by_days() {
    let wrapper = WrapperChrono::new(1);
    let date = WrapperChrono::new_date_utc_now();
    let new_date = wrapper.utc_add_time_by_days();
    assert_eq!(date.day() + wrapper.time as u32, new_date.day());
}

#[test]

fn test_new_date_local_add_time_by_days() {
    let wrapper = WrapperChrono::new(1);
    let date = WrapperChrono::new_date_local_now();
    let new_date = wrapper.local_add_time_by_days();
    assert_eq!(date.day() + wrapper.time as u32, new_date.day());
}

#[test]
fn test_new_date_local_add_time_by_minutes() {
    let wrapper = WrapperChrono::new(1);
    let date = WrapperChrono::new_date_local_now();
    let new_date = wrapper.local_add_time_by_minutes();
    assert_eq!(date.minute() + wrapper.time as u32, new_date.minute());
}

#[test]
fn test_new_date_utc_add_time_by_hours() {
    let wrapper = WrapperChrono::new(1);
    let date = WrapperChrono::new_date_utc_now();
    let new_date = wrapper.utc_add_time_by_hours();
    assert_eq!((date.hour() + wrapper.time as u32) % 24, new_date.hour());
}

#[test]
fn test_new_date_local_add_time_by_hours() {
    let wrapper = WrapperChrono::new(1);
    let date = WrapperChrono::new_date_local_now();
    let new_date = wrapper.local_add_time_by_hours();
    assert_eq!((date.hour() + wrapper.time as u32) % 24, new_date.hour());
}

#[test]
fn test_new_date_utc_add_time_by_weeks() {
    let wrapper = WrapperChrono::new(1);
    let date = WrapperChrono::new_date_utc_now();
    let new_date = wrapper.utc_add_time_by_weeks();
    assert_eq!((new_date - date).num_weeks(), wrapper.time);
}

#[test]
fn test_new_date_local_add_time_by_weeks() {
    let wrapper = WrapperChrono::new(1);
    let date = WrapperChrono::new_date_local_now();
    let new_date = wrapper.local_add_time_by_weeks();
    assert_eq!((new_date - date).num_weeks(), wrapper.time);
}

#[test]
fn test_date_utc_is_expired() {
    let date = WrapperChrono::new_date_utc_now();
    let expiration_date = date - Duration::seconds(1);
    assert!(WrapperChrono::date_utc_is_expired(expiration_date));
}

#[test]
fn test_date_local_is_expired() {
    let date = WrapperChrono::new_date_local_now();
    let expiration_date = date - Duration::seconds(1);
    assert!(WrapperChrono::date_local_is_expired(expiration_date));
}

#[test]
fn test_duration_sec() {
    let wrapper = WrapperChrono::new(1);
    let duration = wrapper.duration_sec();
    assert_eq!(duration.num_seconds(), wrapper.time);
}

#[test]
fn test_duration_minutes() {
    let wrapper = WrapperChrono::new(1);
    let duration = wrapper.duration_minutes();
    assert_eq!(duration.num_minutes(), wrapper.time);
}

#[test]
fn test_duration_hours() {
    let wrapper = WrapperChrono::new(1);
    let duration = wrapper.duration_hours();
    assert_eq!(duration.num_hours(), wrapper.time);
}

#[test]
fn test_duration_days() {
    let wrapper = WrapperChrono::new(1);
    let duration = wrapper.duration_days();
    assert_eq!(duration.num_days(), wrapper.time);
}

#[test]
fn test_duration_weeks() {
    let wrapper = WrapperChrono::new(1);
    let duration = wrapper.duration_weeks();
    assert_eq!(duration.num_weeks(), wrapper.time);
}

#[test]
fn test_new_date_local_add_time_by_seconds() {
    let wrapper = WrapperChrono::new(1);
    let date = WrapperChrono::new_date_local_now();
    let new_date = wrapper.local_add_time_by_seconds();
    assert!((new_date - date).num_seconds() >= wrapper.time);
}

#[test]
fn test_new_date_utc_add_time_by_seconds() {
    let wrapper = WrapperChrono::new(1);
    let date = WrapperChrono::new_date_utc_now();
    let new_date = wrapper.utc_add_time_by_seconds();
    assert!((new_date - date).num_seconds() >= wrapper.time);
}
