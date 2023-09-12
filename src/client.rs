use chrono::TimeZone;

use crate::{datetime::Date, error::Error, jsonrpc, params, resources::*, Session};

/// Client for accessing the Untis API. Can be constructed by [`Client::login()`](Self::login) or [`School::client_login()`](School::client_login).
///
/// # Example
/// ```rust
/// let result = untis::Client::login("server.webuntis.com", "school", "username", "password");
/// match result {
///     Err(err) => println!("{}", err),
///     Ok(client) => {
///         let info = client.session();
///     }
/// }
/// ```
pub struct Client {
    rpc_client: jsonrpc::Client,
    session: Session,
}

impl Client {
    /// Method for creating a new session.
    /// The `server` and `school` parameter both depend on the school that the user is part of; You can get `server` from
    /// [`School.server`](crate::School::server) and `school` from [`School.login_name`](crate::School::login_name).
    pub fn login(
        server: &str,
        school: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, Error> {
        let params = params::AuthenticateParams {
            client: "untis-rs",
            user: username,
            password: password,
        };
        let mut rpc_client = jsonrpc::Client::new(&make_untis_url(server, school));
        let session: Session = rpc_client.request("authenticate", params)?;
        Ok(Self {
            rpc_client,
            session,
        })
    }

    /// Returns the active session.
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// Returns the last time that any timetable at this school was updated.
    pub fn last_update_time(&mut self) -> Result<chrono::DateTime<chrono::Utc>, Error> {
        let ts: i64 = self.rpc_client.request("getLatestImportTime", ())?;
        Ok(chrono::Utc.timestamp_millis_opt(ts).unwrap())
    }

    /// Returns status data that can be used for displaying a timetable.
    pub fn status_data(&mut self) -> Result<StatusData, Error> {
        self.rpc_client.request("getStatusData", ())
    }

    /// Retrieves the current schoolyear.
    pub fn current_schoolyear(&mut self) -> Result<Schoolyear, Error> {
        self.rpc_client.request("getCurrentSchoolyear", ())
    }

    /// Retrieves a list of all schoolyears.
    pub fn schoolyears(&mut self) -> Result<Vec<Schoolyear>, Error> {
        self.rpc_client.request("getSchoolyears", ())
    }

    /// Retrieves the holidays in the current schoolyear.
    pub fn holidays(&mut self) -> Result<Vec<Holiday>, Error> {
        self.rpc_client.request("getHolidays", ())
    }

    /// Retrieves the list of rooms in the user's school.
    pub fn rooms(&mut self) -> Result<Vec<Room>, Error> {
        self.rpc_client.request("getRooms", ())
    }

    /// Retrieves the list of classes in the user's school.
    pub fn classes(&mut self) -> Result<Vec<Class>, Error> {
        self.rpc_client.request("getKlassen", ())
    }

    /// Retrieves the list of subjects in the user's school.
    pub fn subjects(&mut self) -> Result<Vec<Subject>, Error> {
        self.rpc_client.request("getSubjects", ())
    }

    /// Retrieves the list of teachers in the user's school.
    pub fn teachers(&mut self) -> Result<Vec<Teacher>, Error> {
        self.rpc_client.request("getTeachers", ())
    }

    /// Retrieves the list of students in the user's school.
    pub fn students(&mut self) -> Result<Vec<Student>, Error> {
        self.rpc_client.request("getStudents", ())
    }

    /// Retrieves the user's own timetable between now and a given date.
    pub fn own_timetable_until(&mut self, end_date: &Date) -> Result<Vec<Lesson>, Error> {
        self.own_timetable_between(&Date::today(), end_date)
    }

    /// Retrieves the users's own timetable for the current week.
    pub fn own_timetable_current_week(&mut self) -> Result<Vec<Lesson>, Error> {
        self.own_timetable_for_week(&Date::today())
    }

    /// Retrieves the users's own timetable for the week that a given date is in.
    pub fn own_timetable_for_week(&mut self, date: &Date) -> Result<Vec<Lesson>, Error> {
        self.own_timetable_between(&date.relative_week_begin(), &date.relative_week_end())
    }

    /// Retrieves the users's own timetable between two dates.
    pub fn own_timetable_between(
        &mut self,
        start_date: &Date,
        end_date: &Date,
    ) -> Result<Vec<Lesson>, Error> {
        self.timetable_between(
            &self.session.person_id.clone(),
            &self.session.person_type.clone(),
            start_date,
            end_date,
        )
    }

    /// Retrieves an element's timetable between now and a given date.
    pub fn timetable_until(
        &mut self,
        id: &usize,
        ty: &ElementType,
        end_date: &Date,
    ) -> Result<Vec<Lesson>, Error> {
        self.timetable_between(id, ty, &Date::today(), end_date)
    }

    /// Retrieves an element's timetable for the current week.
    pub fn timetable_current_week(
        &mut self,
        id: &usize,
        ty: &ElementType,
    ) -> Result<Vec<Lesson>, Error> {
        self.timetable_for_week(id, ty, &Date::today())
    }

    /// Retrieves an element's timetable for the week that a given date is in.
    pub fn timetable_for_week(
        &mut self,
        id: &usize,
        ty: &ElementType,
        date: &Date,
    ) -> Result<Vec<Lesson>, Error> {
        self.timetable_between(
            id,
            ty,
            &date.relative_week_begin(),
            &date.relative_week_end(),
        )
    }

    /// Retrieves an element's own timetable between two dates.
    pub fn timetable_between(
        &mut self,
        id: &usize,
        ty: &ElementType,
        start_date: &Date,
        end_date: &Date,
    ) -> Result<Vec<Lesson>, Error> {
        let params = params::TimetableParams {
            options: &params::TimetableParamsOpts {
                element: &params::TimetableParamsElem { id, ty },
                start_date,
                end_date,
                show_booking: &true,
                show_info: &true,
                show_subst_text: &true,
                show_ls_text: &true,
                show_ls_number: &true,
                show_student_group: &true,
                class_fields: &["id", "name"],
                room_fields: &["id", "name"],
                subject_fields: &["id", "name"],
                teacher_fields: &["id", "name"],
            },
        };
        self.rpc_client.request("getTimetable", params)
    }

    /// Retrieves the list of departments in the user's school.
    pub fn departments(&mut self) -> Result<Vec<Department>, Error> {
        self.rpc_client.request("getDepartments", ())
    }

    fn logout(&mut self) -> Result<(), Error> {
        self.rpc_client.request("logout", ())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        _ = self.logout();
    }
}

impl School {
    pub fn client_login(&self, username: &str, password: &str) -> Result<Client, Error> {
        Client::login(&self.server, &self.login_name, username, password)
    }
}

fn make_untis_url(server: &str, school: &str) -> String {
    format!("https://{}/WebUntis/jsonrpc.do?school={}", server, school)
}
