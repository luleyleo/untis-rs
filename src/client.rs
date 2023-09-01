use crate::{datetime::Date, error::Error, jsonrpc, params, resources::*, Session};

/// Client for accessing the Untis API. Can be constructed by [`Client::login()`](Self::login) or [`School::client_login()`](School::client_login).
/// Please call [`client.logout()`](Self::logout)) after you're done to free resources on Untis' servers.
///
/// # Example
/// ```rust
/// # fn main() {
/// let result = untis::Client::login("server.webuntis.com", "school", "username", "password");
/// match result {
///     Err(err) => println!("{}", err),
///     Ok(client) => {
///         let info = client.session();
///         client.logout();
///     }
/// }
/// # }
/// ```
pub struct Client {
    rpc_client: jsonrpc::Client,
    session: Session,
}

impl Client {
    /// Method for creating a new session.
    /// The `server` parameter represents the domain that the school's Untis instance is hosted on, like `ikarus.webuntis.com`.
    /// One way to find out a school's server is using the [`untis::schools`](crate::schools) module.
    ///
    /// You can get the `school` parameter from [`School.login_name`](crate::School#structfield.login_name).
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

    /// Returns status data that can be used for displaying a timetable.
    pub fn status_data(&mut self) -> Result<StatusData, Error> {
        self.rpc_client.request("getStatusData", ())
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
    pub fn own_timetable_until(&mut self, end_date: &Date) -> Result<Timetable, Error> {
        self.own_timetable_between(&Date::today(), end_date)
    }

    /// Retrieves the users's own timetable for the current week.
    pub fn own_timetable_current_week(&mut self) -> Result<Timetable, Error> {
        self.own_timetable_for_week(&Date::today())
    }

    /// Retrieves the users's own timetable for the week that a given date is in.
    pub fn own_timetable_for_week(&mut self, date: &Date) -> Result<Timetable, Error> {
        self.own_timetable_between(&date.relative_week_begin(), &date.relative_week_end())
    }

    /// Retrieves the users's own timetable between two dates.
    pub fn own_timetable_between(
        &mut self,
        start_date: &Date,
        end_date: &Date,
    ) -> Result<Timetable, Error> {
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
    ) -> Result<Timetable, Error> {
        self.timetable_between(id, ty, &Date::today(), end_date)
    }

    /// Retrieves an element's timetable for the current week.
    pub fn timetable_current_week(
        &mut self,
        id: &usize,
        ty: &ElementType,
    ) -> Result<Timetable, Error> {
        self.timetable_for_week(id, ty, &Date::today())
    }

    /// Retrieves an element's timetable for the week that a given date is in.
    pub fn timetable_for_week(
        &mut self,
        id: &usize,
        ty: &ElementType,
        date: &Date,
    ) -> Result<Timetable, Error> {
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
    ) -> Result<Timetable, Error> {
        let params = params::TimetableParams {
            id,
            ty,
            start_date,
            end_date,
        };
        self.rpc_client.request("getTimetable", params)
    }

    /// Retrieves the list of departments in the user's school.
    pub fn departments(&mut self) -> Result<Vec<Department>, Error> {
        self.rpc_client.request("getDepartments", ())
    }

    /// Quits the current session.
    pub fn logout(mut self) -> Result<(), Error> {
        self.rpc_client.request("logout", ())
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
