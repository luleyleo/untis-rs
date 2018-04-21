use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use chrono::NaiveTime;
use std::ops::Deref;
use std::fmt;

#[derive(Debug)]
pub struct UntisTime(NaiveTime);

impl Deref for UntisTime {
    type Target = NaiveTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for UntisTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(chrono_to_untis_time(**self))
    }
}

impl<'de> Deserialize<'de> for UntisTime {
    fn deserialize<D>(deserializer: D) -> Result<UntisTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u32(UntisTimeVisitor)
    }
}

struct UntisTimeVisitor;

impl<'de> Visitor<'de> for UntisTimeVisitor {
    type Value = UntisTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .write_str("an integer in the format YEARMONTHDAY like 20180316 for the 16. March 2018")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisTime(chrono_from_untis_time(value as u32)))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisTime(chrono_from_untis_time(value as u32)))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisTime(chrono_from_untis_time(value as u32)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisTime(chrono_from_untis_time(value as u32)))
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisTime(chrono_from_untis_time(value as u32)))
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisTime(chrono_from_untis_time(value as u32)))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisTime(chrono_from_untis_time(value as u32)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where E: serde::de::Error,
    {
        Ok(UntisTime(chrono_from_untis_time(value as u32)))
    }
}

fn chrono_to_untis_time(time: NaiveTime) -> u32 {
    let string = format!("{}", time.format("%k%M"));
    let number = string.trim().parse::<u32>().unwrap();
    number
}

fn chrono_from_untis_time(value: u32) -> NaiveTime {
    let string = format!("{}", value);
    let hour_len = if string.len() == 3 { 1 } else { 2 };
    let hours = string[0..hour_len].parse::<u32>().unwrap();
    let mins = string[hour_len..(hour_len+2)].parse::<u32>().unwrap();

    NaiveTime::from_hms(hours, mins, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_untis_time_forth_and_back() {
        let number = 830;
        let time = chrono_from_untis_time(number);
        println!("Time: {}", time);
        let new_number = chrono_to_untis_time(time);
        println!("Num:  {}", new_number);

        assert_eq!(number, new_number);
    }
}
