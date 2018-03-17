# untis-rs

Rust library to access the webuntis jsonrpc

```rust
fn main() {
    let mut untis = Units::new("server", "school", "user", "password");
    
    let info = untis.login().expect("Failed to login");

    let _statusdata  = untis.status_data()                                  .expect("Failed to get status data" );
    let _holidays    = untis.holidays()                                     .expect("Failed to get holidays"    );
    let _rooms       = untis.rooms()                                        .expect("Failed to get rooms"       );
    let _classes     = untis.classes()                                      .expect("Failed to get classes"     );
    let _subjects    = untis.subjects()                                     .expect("Failed to get subjects"    );
    let _timetable   = untis.timetable(info.class_id, 1, Local::today())    .expect("Failed to get timetable"   );
    let _departments = untis.departments()                                  .expect("Failed to get departments" );
    // teachers

    untis.logout().expect("Failed to logout");
}
```