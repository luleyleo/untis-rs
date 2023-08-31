use chrono::NaiveDate;

use crate::date::UntisDate;

#[derive(Serialize)]
pub struct ParamsAuthenticate<'a> {
    client: &'a str,
    user: &'a str,
    password: &'a str,
}

impl<'a> ParamsAuthenticate<'a> {
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
pub struct ParamsTimetable {
    id: usize,

    #[serde(rename = "type")]
    ty: usize,

    start_date: UntisDate,
    end_date: UntisDate,
}

impl ParamsTimetable {
    pub fn new(id: usize, ty: usize, date: NaiveDate) -> Self {
        Self {
            id,
            ty,
            start_date: UntisDate::week_begin_from(date),
            end_date: UntisDate::week_end_from(date),
        }
    }
}
