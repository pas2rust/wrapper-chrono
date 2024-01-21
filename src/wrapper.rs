use chrono::{DateTime, Duration, Local, Utc};
use darth_rust::DarthRust;

#[derive(Debug, DarthRust)]
pub struct WrapperChrono {
    pub time: i64,
}
pub trait ChronoTrait {
    fn utc_add_time_by_weeks(&self) -> DateTime<Utc>;
    fn utc_add_time_by_days(&self) -> DateTime<Utc>;
    fn utc_add_time_by_hours(&self) -> DateTime<Utc>;
    fn utc_add_time_by_minutes(&self) -> DateTime<Utc>;
    fn utc_add_time_by_seconds(&self) -> DateTime<Utc>;
    fn local_add_time_by_weeks(&self) -> DateTime<Local>;
    fn local_add_time_by_days(&self) -> DateTime<Local>;
    fn local_add_time_by_hours(&self) -> DateTime<Local>;
    fn local_add_time_by_minutes(&self) -> DateTime<Local>;
    fn local_add_time_by_seconds(&self) -> DateTime<Local>;
    fn date_utc_is_expired(expiration_date: DateTime<Utc>) -> bool;
    fn date_local_is_expired(expiration_date: DateTime<Local>) -> bool;
    fn new_date_utc_now() -> DateTime<Utc>;
    fn new_date_local_now() -> DateTime<Local>;
    fn duration_sec(&self) -> Duration;
    fn duration_hours(&self) -> Duration;
    fn duration_minutes(&self) -> Duration;
    fn duration_days(&self) -> Duration;
    fn duration_weeks(&self) -> Duration;
}

impl ChronoTrait for WrapperChrono {
    fn duration_sec(&self) -> Duration {
        let time = self.time;
        Duration::seconds(time)
    }
    fn duration_hours(&self) -> Duration {
        let time = self.time;
        Duration::hours(time)
    }
    fn duration_days(&self) -> Duration {
        let time = self.time;
        Duration::days(time)
    }
    fn duration_minutes(&self) -> Duration {
        let time = self.time;
        Duration::minutes(time)
    }
    fn duration_weeks(&self) -> Duration {
        let time = self.time;
        Duration::weeks(time)
    }
    fn utc_add_time_by_hours(&self) -> DateTime<Utc> {
        let now = Self::new_date_utc_now();
        let duration = self.duration_hours();
        now + duration
    }
    fn utc_add_time_by_minutes(&self) -> DateTime<Utc> {
        let now = Self::new_date_utc_now();
        let duration = self.duration_minutes();
        now + duration
    }
    fn utc_add_time_by_seconds(&self) -> DateTime<Utc> {
        let now = Self::new_date_utc_now();
        let duration = self.duration_sec();
        now + duration
    }
    fn utc_add_time_by_weeks(&self) -> DateTime<Utc> {
        let now = Self::new_date_utc_now();
        let duration = self.duration_weeks();
        now + duration
    }
    fn utc_add_time_by_days(&self) -> DateTime<Utc> {
        let now = Self::new_date_utc_now();
        let duration = self.duration_days();
        now + duration
    }
    fn local_add_time_by_minutes(&self) -> DateTime<Local> {
        let now = Self::new_date_local_now();
        let duration = self.duration_minutes();
        now + duration
    }
    fn local_add_time_by_weeks(&self) -> DateTime<Local> {
        let now = Self::new_date_local_now();
        let duration = self.duration_weeks();
        now + duration
    }
    fn local_add_time_by_seconds(&self) -> DateTime<Local> {
        let now = Self::new_date_local_now();
        let duration = self.duration_sec();
        now + duration
    }
    fn local_add_time_by_hours(&self) -> DateTime<Local> {
        let now = Self::new_date_local_now();
        let duration = self.duration_hours();
        now + duration
    }
    fn local_add_time_by_days(&self) -> DateTime<Local> {
        let now = Self::new_date_local_now();
        let duration = self.duration_days();
        now + duration
    }
    fn new_date_local_now() -> DateTime<Local> {
        Local::now()
    }
    fn new_date_utc_now() -> DateTime<Utc> {
        Utc::now()
    }
    fn date_utc_is_expired(expiration_date: DateTime<Utc>) -> bool {
        let now = Self::new_date_utc_now();
        now > expiration_date
    }
    fn date_local_is_expired(expiration_date: DateTime<Local>) -> bool {
        let now = Self::new_date_local_now();
        now > expiration_date
    }
}
