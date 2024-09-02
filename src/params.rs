use crate::{datetime::Date, ElementType};
use serde::Serialize;

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
    pub options: &'a TimetableParamsOpts<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimetableParamsOpts<'a> {
    pub element: &'a TimetableParamsElem<'a>,
    pub start_date: &'a Date,
    pub end_date: &'a Date,
    pub show_booking: &'a bool,
    pub show_info: &'a bool,
    pub show_subst_text: &'a bool,
    pub show_ls_text: &'a bool,
    pub show_ls_number: &'a bool,
    pub show_student_group: &'a bool,
    #[serde(rename = "klasseFields")]
    pub class_fields: &'a [&'a str],
    pub room_fields: &'a [&'a str],
    pub subject_fields: &'a [&'a str],
    pub teacher_fields: &'a [&'a str],
}

#[derive(Serialize)]
pub struct TimetableParamsElem<'a> {
    pub id: &'a usize,
    #[serde(rename = "type")]
    pub ty: &'a ElementType,
}
