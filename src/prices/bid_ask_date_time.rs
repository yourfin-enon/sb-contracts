#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidAskDateTime {
    /// the offset (in units of the selected scale) from 1970/01/01
    #[prost(sint64, tag = "1")]
    pub value: i64,
    /// the scale of the timespan [default = DAYS]
    #[prost(enumeration = "date_time::TimeSpanScale", tag = "2")]
    pub scale: i32,
    /// the kind of date/time being represented [default = UNSPECIFIED]
    #[prost(enumeration = "date_time::DateTimeKind", tag = "3")]
    pub kind: i32,
}

impl BidAskDateTime {
    pub fn from_millis(millis: i64) -> Self {
        Self {
            value: millis,
            scale: 4,
            kind: 1,
        }
    }

    pub fn timestamp_sec(&self) -> i64 {
        let scale = date_time::TimeSpanScale::try_from(self.scale)
            .expect("Failed to parse scale to TimeSpanScale");

        match scale {
            date_time::TimeSpanScale::Seconds => self.value,
            date_time::TimeSpanScale::Milliseconds => self.value / 1000,
            date_time::TimeSpanScale::Ticks => self.value / 10_000_000,
            date_time::TimeSpanScale::Minutes => self.value * 60,
            date_time::TimeSpanScale::Hours => self.value * 3600,
            date_time::TimeSpanScale::Days => self.value * 86400,
            _ => panic!("Not supported scale"),
        }
    }

    pub fn timestamp_micros(&self) -> i64 {
        let scale = date_time::TimeSpanScale::try_from(self.scale)
            .expect("Failed to parse scale to TimeSpanScale");

        match scale {
            date_time::TimeSpanScale::Ticks => self.value / 10,
            date_time::TimeSpanScale::Seconds => self.value * 1_000_00,
            date_time::TimeSpanScale::Milliseconds => self.value * 1_000,
            date_time::TimeSpanScale::Minutes => self.value * 60_000_000,
            date_time::TimeSpanScale::Hours => self.value * 3_600_000_000,
            date_time::TimeSpanScale::Days => self.value * 86_400_000_000,
            _ => panic!("Not supported scale"),
        }
    }

    pub fn timestamp_millis(&self) -> i64 {
        let scale = date_time::TimeSpanScale::try_from(self.scale)
            .expect("Failed to parse scale to TimeSpanScale");

        match scale {
            date_time::TimeSpanScale::Seconds => self.value * 1000,
            date_time::TimeSpanScale::Milliseconds => self.value,
            date_time::TimeSpanScale::Ticks => self.value / 10_000,
            date_time::TimeSpanScale::Minutes => self.value * 60_000,
            date_time::TimeSpanScale::Hours => self.value * 3_600_000,
            date_time::TimeSpanScale::Days => self.value * 86_400_000,
            _ => panic!("Not supported scale"),
        }
    }
}

/// Nested message and enum types in `DateTime`.
pub mod date_time {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum TimeSpanScale {
        Days = 0,
        Hours = 1,
        Minutes = 2,
        Seconds = 3,
        Milliseconds = 4,
        Ticks = 5,
        /// dubious
        Minmax = 15,
    }
    impl TimeSpanScale {
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeSpanScale::Days => "DAYS",
                TimeSpanScale::Hours => "HOURS",
                TimeSpanScale::Minutes => "MINUTES",
                TimeSpanScale::Seconds => "SECONDS",
                TimeSpanScale::Milliseconds => "MILLISECONDS",
                TimeSpanScale::Ticks => "TICKS",
                TimeSpanScale::Minmax => "MINMAX",
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DateTimeKind {
        /// The time represented is not specified as either local time or Coordinated Universal Time (UTC).
        Unspecified = 0,
        Utc = 1,
        Local = 2,
    }
    impl DateTimeKind {
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DateTimeKind::Unspecified => "UNSPECIFIED",
                DateTimeKind::Utc => "UTC",
                DateTimeKind::Local => "LOCAL",
            }
        }
    }
}
