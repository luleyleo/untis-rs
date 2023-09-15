use crate::datetime::{Date, Time};
use serde::de::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

/// The different types of elements that exist in the Untis API.
#[derive(Serialize_repr, Deserialize_repr, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum ElementType {
    Class = 1,
    Teacher,
    Subject,
    Room,
    Student,
    #[serde(other)]
    Other,
}

impl ElementType {
    pub fn as_u8(&self) -> u8 {
        self.clone() as u8
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(crate) struct SchoolSearchResult {
    pub size: usize,
    pub schools: Vec<School>,
}

/// A school that uses Untis.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct School {
    /// The Untis instance that this school uses, e.g. `ajax.webuntis.com`
    pub server: String,

    pub use_mobile_service_url_android: bool,

    /// The school's address.
    pub address: String,

    /// The school's full name.
    pub display_name: String,

    /// A unique, shorter name for this school.
    pub login_name: String,

    /// This school's unique id in Untis.
    #[serde(rename = "schoolId")]
    pub id: usize,

    pub use_mobile_service_url_ios: bool,

    /// URL of the WebUntis login page for this school.
    pub server_url: String,

    pub mobile_service_url: Option<String>,
}

/// A Untis session.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// The session's id.
    pub session_id: String,

    /// Id of the user's class.
    #[serde(rename = "klasseId")]
    pub class_id: usize,

    /// The user's id.
    pub person_id: usize,

    /// The user's element type (Teacher or Student).
    pub person_type: ElementType,
}

/// A set of colors that can be used to display a timetable.
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct StatusData {
    /// Color information for different lesson types.
    pub lstypes: Vec<HashMap<String, StatusDataItem>>,

    /// Color information for lesson statuses.
    pub codes: Vec<HashMap<String, StatusDataItem>>,
}

/// Color information to display a specific lesson.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDataItem {
    /// Foreground color, formatted as `RRGGBB`.
    pub fore_color: String,

    /// Background color, formatted as `RRGGBB`.
    pub back_color: String,
}

/// A schoolyear.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schoolyear {
    /// The schoolyears's id, unique within this school.
    pub id: usize,

    /// The schoolyear's name.
    pub name: String,

    /// The schoolyear's start date.
    pub start_date: Date,

    /// The schoolyear's end date.
    pub end_date: Date,
}

/// A school holiday.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holiday {
    /// The holiday's id, unique within this school.
    pub id: usize,

    /// The holiday's shortened name, presumably unique within this school.
    pub name: String,

    /// The holiday's full name.
    pub long_name: String,

    /// The holiday's start date.
    pub start_date: Date,

    /// The holiday's end date.
    pub end_date: Date,
}

/// Represents a room for school lessons.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    /// The room's id, unique within this school.
    pub id: usize,

    /// The room's shortened name, unique within this school.
    pub name: String,

    /// The room's full name.
    pub long_name: String,

    /// Whether the room is generally available or not used in the system.
    pub active: bool,

    /// Foreground color for displaying the room, formatted as `RRGGBB`.
    pub fore_color: Option<String>,

    /// Background color for displaying the room, formatted as `RRGGBB`.
    pub back_color: Option<String>,

    /// The building that this room is located in. May be an empty string if you school hasn't configured any.
    pub building: String,

    pub did: Option<usize>,
}

/// Represents a school class.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    /// The class's id, unique within this school.
    pub id: usize,

    /// The class's shortened name, unique within this school.
    pub name: String,

    /// The class's full name.
    pub long_name: String,

    /// Whether the class is generally available or not used in the system.
    pub active: bool,

    /// Foreground color for displaying the class, formatted as `RRGGBB`.
    pub fore_color: Option<String>,

    /// Background color for displaying the class, formatted as `RRGGBB`.
    pub back_color: Option<String>,

    pub did: Option<usize>,

    /// Id of the class's primary teacher. May be -1 if there is none.
    #[serde(default = "default_id")]
    pub teacher1: isize,

    /// Id of the class's secondary teacher. May be -1 if there is none.
    #[serde(default = "default_id")]
    pub teacher2: isize,
}

/// Represents a school subject.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    /// The subject's id, unique within this school.
    pub id: usize,

    /// The subject's shortened name, unique within this school.
    pub name: String,

    /// The subject's full name.
    pub long_name: String,

    pub alternate_name: String,

    /// Whether the subject is generally available or not used in the system.
    pub active: bool,

    /// Foreground color for displaying the subject, formatted as `RRGGBB`.
    pub fore_color: Option<String>,

    /// Background color for displaying the subject, formatted as `RRGGBB`.
    pub back_color: Option<String>,
}

/// Represents a teacher.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
    /// The teacher's id, unique within this school.
    pub id: usize,

    /// The teacher's shortened name, unique within this school.
    pub name: String,

    /// The teacher's first name.
    #[serde(rename = "foreName")]
    pub first_name: String,

    /// The teacher's last name.
    #[serde(rename = "longName")]
    pub last_name: String,

    /// The teacher's title.
    pub title: String,

    /// Whether the teacher is generally available or not used in the system.
    pub active: bool,

    pub dids: Vec<DidItem>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct DidItem {
    id: usize,
}

/// Represents a student.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    /// The student's id, unique within this school.
    pub id: usize,

    pub key: String,

    /// The student's shortened name, unique within this school.
    pub name: String,

    /// The student's first name.
    #[serde(rename = "foreName")]
    pub first_name: String,

    /// The student's last name.
    #[serde(rename = "longName")]
    pub last_name: String,

    /// The student's gender.
    pub gender: String,
}

/// A school lesson.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    /// The lesson's id.
    pub id: usize,

    /// The lesson's date.
    pub date: Date,

    /// The lesson's start time.
    pub start_time: Time,

    /// The lesson's end time.
    pub end_time: Time,

    /// The type of lesson.
    #[serde(rename = "lstype", default)]
    pub lesson_type: LessonType,

    /// The lesson's code.
    #[serde(default)]
    pub code: LessonCode,

    /// Unique id for this specific schedule.
    pub lsnumber: usize,

    /// Info Text for this specific lesson.
    #[serde(default)]
    pub lstext: String,

    /// Possible substitution text.
    pub subst_text: Option<String>,

    /// The classes that are part of this lesson.
    #[serde(rename = "kl")]
    pub classes: Vec<IdItem>,

    /// The subjects that are taught in this lesson.
    #[serde(rename = "su")]
    pub subjects: Vec<IdItem>,

    /// The rooms that this lesson takes place in.
    #[serde(rename = "ro")]
    pub rooms: Vec<IdItem>,

    #[serde(default)]
    pub statflags: String,

    /// The lesson's activity type.
    #[serde(default = "default_activity_type")]
    pub activity_type: String,
}

/// Represents the status of a lesson (regular, cancelled, etc.)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug, Serialize)]
pub enum LessonCode {
    #[default]
    Regular,
    Irregular,
    Cancelled,
}

impl<'de> Deserialize<'de> for LessonCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "irregular" => LessonCode::Irregular,
            "cancelled" => LessonCode::Cancelled,
            "regular" | _ => LessonCode::Regular,
        })
    }
}

/// Represents the type of lesson.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Debug, Serialize)]
pub enum LessonType {
    #[default]
    Lesson,
    OfficeHour,
    Standby,
    BreakSupervision,
    Exam,
}

impl<'de> Deserialize<'de> for LessonType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "oh" => LessonType::OfficeHour,
            "sb" => LessonType::Standby,
            "bs" => LessonType::BreakSupervision,
            "ex" => LessonType::Exam,
            "ls" | _ => LessonType::Lesson,
        })
    }
}

/// Represents an element that is part of a lesson.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct IdItem {
    /// The element's id.
    pub id: isize,

    /// The element's short name.
    pub name: String,

    /// If this element is a substitute, this is the id of the original element.
    #[serde(rename = "original_id")]
    pub orgid: Option<isize>,
}

/// Represents a school department.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Department {
    /// The department's id, unique within this school.
    pub id: usize,

    /// The department's short name, unique within this school.
    pub name: String,

    /// The department's full name.
    pub long_name: String,
}

fn default_id() -> isize {
    -1
}

fn default_activity_type() -> String {
    String::from("undefined")
}
