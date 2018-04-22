use std::collections::HashMap;
use date::UntisDate;
use time::UntisTime;

#[derive(Debug, Deserialize)]
pub struct RpcResponse<R> {
    pub jsonrpc: String,
    pub id: String,
    pub result: R,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    pub session_id: String,

    #[serde(rename = "klasseId")]
    pub class_id: usize,

    pub person_id: usize,
    pub person_type: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusData {
    pub lstypes: Vec<HashMap<String, StatusDataItem>>,
    pub codes: Vec<HashMap<String, StatusDataItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDataItem {
    pub fore_color: String,
    pub back_color: String,
}

pub type Holidays = Vec<HolidaysItem>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolidaysItem {
    pub id: usize,
    pub name: String,
    pub long_name: String,
    pub start_date: UntisDate,
    pub end_date: UntisDate,
}

pub type Rooms = Vec<RoomItem>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomItem {
    pub id: usize,
    pub name: String,
    pub long_name: String,
    pub active: bool,
    pub fore_color: Option<String>,
    pub back_color: Option<String>,
    pub building: String,
    pub did: Option<usize>,
}

pub type Classes = Vec<ClassItem>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassItem {
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

pub type Subjects = Vec<SubjectItem>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectItem {
    pub id: usize,
    pub name: String,
    pub long_name: String,
    pub alternate_name: String,
    pub active: bool,
    pub fore_color: Option<String>,
    pub back_color: Option<String>,
}

pub type Timetable = Vec<TimetableItem>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimetableItem {
    pub id: usize,
    pub date: UntisDate,
    pub start_time: UntisTime,
    pub end_time: UntisTime,

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

#[derive(Debug, Serialize, Deserialize)]
pub struct IdItem {
    pub id: isize,

    pub orgid: Option<isize>,
}

pub type Departments = Vec<DepartmentItem>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentItem {
    pub id: usize,
    pub name: String,
    pub long_name: String,
}

fn default_id() -> isize { -1 }
fn default_activity_type() -> String { "undefined".to_owned() }
