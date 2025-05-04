//! A place where common types reside

/// Supported export file extensions
#[derive(Debug, Default, PartialEq, Clone)]
pub enum FileExt {
    #[default]
    Xlsx,
    Csv,
}

#[derive(Debug, Default, Clone)]
pub enum Day {
    #[default]
    Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday,
}

#[derive(Debug, Default, Clone)]
pub enum Month {
    #[default]
    January, February, March, April, May, June, July, August, September,
    October, November, December,
}

impl Day {
    pub fn from(day: &str) -> Self {
        let day_uc = day.to_uppercase();
        let day_str = day_uc.as_str();

        match day_str {
            "SUNDAY" | "SUN" | "SU" => Day::Sunday,
            "MONDAY" | "MON" | "MO" | "M" => Day::Monday,
            "TUESDAY" | "TUE" | "TU" => Day::Tuesday,
            "WEDNESDAY" | "WED" | "WE" | "W" => Day::Wednesday,
            "THURSDAY" | "THU" | "TH" => Day::Thursday,
            "FRIDAY" | "FRI" | "FR" | "F" => Day::Friday,
            "SATURDAY" | "SAT" | "SA" => Day::Saturday,

            _ => Day::Sunday,
        }
    }

    pub fn from_u32(num: u32) -> Self {
        match num {
            0 => Day::Sunday,
            1 => Day::Monday,
            2 => Day::Tuesday,
            3 => Day::Wednesday,
            4 => Day::Thursday,
            5 => Day::Friday,
            6 => Day::Saturday,

            _ => Day::Sunday,
        }
    }

    pub fn as_str<'a>(&self) -> &str {
        match self {
            Day::Sunday => "Sunday",
            Day::Monday => "Monday",
            Day::Tuesday => "Tuesday",
            Day::Wednesday => "Wednesday",
            Day::Thursday => "Thursday",
            Day::Friday => "Friday",
            Day::Saturday => "Saturday",
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Day::Sunday => 0,
            Day::Monday => 1,
            Day::Tuesday => 2,
            Day::Wednesday => 3,
            Day::Thursday => 4,
            Day::Friday => 5,
            Day::Saturday => 6,
        }
    }
}

impl Month {
    pub fn from_u32(num: u32) -> Self {
        match num {
            1 => Self::January,
            2 => Self::February,
            3 => Self::March,
            4 => Self::April,
            5 => Self::May,
            6 => Self::June,
            7 => Self::July,
            8 => Self::August,
            9 => Self::September,
            10 => Self::October,
            11 => Self::November,
            12 => Self::December,
            _ => Self::January,
        }
    }

    pub fn as_str<'a>(&self) -> &str {
        match self {
            Self::January => "January",
            Self::February => "February",
            Self::March => "March",
            Self::April => "April",
            Self::May => "May",
            Self::June => "June",
            Self::July => "July",
            Self::August => "August",
            Self::September => "September",
            Self::October => "October",
            Self::November => "November",
            Self::December => "December",
        }
    }
}

