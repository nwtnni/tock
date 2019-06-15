#[derive(Copy, Clone, Debug)]
pub struct Tz;

impl Tz {
    pub fn name(&self) -> &'static str {
        "UTC"
    }
}

impl chrono::TimeZone for Tz {
    type Offset = chrono::Utc;

    fn from_offset(_: &chrono::Utc) -> Self { Tz }

    fn offset_from_local_date(&self, _: &chrono::NaiveDate) -> chrono::LocalResult<chrono::Utc> {
        chrono::LocalResult::Single(chrono::Utc)
    }

    fn offset_from_local_datetime(&self, _: &chrono::NaiveDateTime) -> chrono::LocalResult<chrono::Utc> {
        chrono::LocalResult::Single(chrono::Utc)
    }

    fn offset_from_utc_date(&self, _: &chrono::NaiveDate) -> chrono::Utc {
        chrono::Utc
    }

    fn offset_from_utc_datetime(&self, _: &chrono::NaiveDateTime) -> chrono::Utc {
        chrono::Utc
    }
}

impl std::str::FromStr for Tz {
    type Err = String; 
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
        | "UTC" 
        | "Utc"
        | "utc" => Ok(Tz),
        | s => Err(format!("'{}' is not a valid timezone", s)),
        }
    }
}
