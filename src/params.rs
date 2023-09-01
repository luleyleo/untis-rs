use crate::{datetime::Date, ElementType};

#[derive(Serialize)]
#[serde(untagged)]
pub enum FindSchoolParams<'a> {
    Search { search: &'a str },
    ById { schoolid: &'a usize },
    ByName { schoolname: &'a str },
}

#[derive(Serialize)]
pub struct AuthenticateParams<'a> {
    pub client: &'static str,
    pub user: &'a str,
    pub password: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimetableParams<'a> {
    pub id: &'a usize,

    #[serde(rename = "type")]
    pub ty: &'a ElementType,

    pub start_date: &'a Date,
    pub end_date: &'a Date,
}
