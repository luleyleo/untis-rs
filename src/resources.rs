use crate::datetime::{Date, Time};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

#[derive(
    Serialize_repr, Deserialize_repr, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
#[repr(u8)]
pub enum ElementType {
    Class = 1,
    Teacher,
    Subject,
    Room,
    Student,
}

impl ElementType {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(crate) struct SchoolSearchResult {
    pub size: usize,
    pub schools: Vec<School>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct School {
    pub server: String,
    pub use_mobile_service_url_android: bool,
    pub address: String,
    pub display_name: String,
    pub login_name: String,
    #[serde(rename = "schoolId")]
    pub id: usize,
    pub use_mobile_service_url_ios: bool,
    pub server_url: String,
    pub mobile_service_url: Option<String>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub session_id: String,

    #[serde(rename = "klasseId")]
    pub class_id: usize,

    pub person_id: usize,
    pub person_type: ElementType,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct StatusData {
    pub lstypes: Vec<HashMap<String, StatusDataItem>>,
    pub codes: Vec<HashMap<String, StatusDataItem>>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDataItem {
    pub fore_color: String,
    pub back_color: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holiday {
    pub id: usize,
    pub name: String,
    pub long_name: String,
    pub start_date: Date,
    pub end_date: Date,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: usize,
    pub name: String,
    pub long_name: String,
    pub active: bool,
    pub fore_color: Option<String>,
    pub back_color: Option<String>,
    pub building: String,
    pub did: Option<usize>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub id: usize,
    pub name: String,
    pub long_name: String,
    pub active: bool,
    pub fore_color: Option<String>,
    pub back_color: Option<String>,
    pub did: Option<usize>,

    #[serde(default = "default_id")]
    pub teacher1: isize,

    #[serde(default = "default_id")]
    pub teacher2: isize,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub id: usize,
    pub name: String,
    pub long_name: String,
    pub alternate_name: String,
    pub active: bool,
    pub fore_color: Option<String>,
    pub back_color: Option<String>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
    pub id: usize,
    pub name: String,

    #[serde(rename = "foreName")]
    pub first_name: String,

    #[serde(rename = "longName")]
    pub last_name: String,

    pub title: String,
    pub active: bool,
    pub dids: Vec<IdItem>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    pub id: usize,
    pub key: String,
    pub name: String,

    #[serde(rename = "foreName")]
    pub first_name: String,

    #[serde(rename = "longName")]
    pub last_name: String,

    pub gender: String,
}

pub type Timetable = Vec<Lesson>;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    pub id: usize,
    pub date: Date,
    pub start_time: Time,
    pub end_time: Time,

    #[serde(default)]
    pub code: String,

    #[serde(rename = "kl")]
    pub classes: Vec<IdItem>,

    #[serde(rename = "su")]
    pub subjects: Vec<IdItem>,

    #[serde(rename = "ro")]
    pub rooms: Vec<IdItem>,

    #[serde(default)]
    pub statflags: String,

    #[serde(default = "default_activity_type")]
    pub activity_type: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct IdItem {
    pub id: isize,
    pub orgid: Option<isize>,
}

pub type Departments = Vec<Department>;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Department {
    pub id: usize,
    pub name: String,
    pub long_name: String,
}

fn default_id() -> isize {
    -1
}

fn default_activity_type() -> String {
    String::from("undefined")
}
