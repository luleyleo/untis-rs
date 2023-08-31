use chrono::NaiveDate;

use crate::{datetime::Date, ElementType};

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum FindSchool<'a> {
    Search { search: &'a str },
    ById { schoolid: usize },
    ByName { schoolname: &'a str },
}

#[derive(Serialize)]
pub struct Authenticate<'a> {
    client: &'a str,
    user: &'a str,
    password: &'a str,
}

impl<'a> Authenticate<'a> {
    pub fn new(client: &'a str, user: &'a str, password: &'a str) -> Self {
        Self {
            client,
            user,
            password,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timetable {
    id: usize,

    #[serde(rename = "type")]
    ty: ElementType,

    start_date: Date,
    end_date: Date,
}

impl Timetable {
    pub fn new(id: usize, ty: ElementType, date: NaiveDate) -> Self {
        Self {
            id,
            ty,
            start_date: Date::week_begin_from(date),
            end_date: Date::week_end_from(date),
        }
    }
}
