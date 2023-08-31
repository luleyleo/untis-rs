extern crate chrono;
extern crate untis;

use chrono::Local;
use untis::Client;

fn main() {
    let mut client = Client::new("server.webuntis.com", "school", "user", "password");
    let today = Local::now().date_naive();

    let info = client.login().expect("Failed to login");

    let _statusdata = client.status_data().expect("Failed to get status data");
    let _holidays = client.holidays().expect("Failed to get holidays");
    let _rooms = client.rooms().expect("Failed to get rooms");
    let _classes = client.classes().expect("Failed to get classes");
    let _subjects = client.subjects().expect("Failed to get subjects");
    let _timetable = client
        .timetable(info.class_id, untis::ElementType::Class, today)
        .expect("Failed to get timetable");
    let _departments = client.departments().expect("Failed to get departments");

    client.logout().expect("Failed to logout");
}
