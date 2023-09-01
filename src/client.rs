use crate::{datetime::Date, error::Error, jsonrpc, params, resources::*, Session};

/// Provides access to webuntis.com
pub struct Client {
    rpc_client: jsonrpc::Client,
    session: Session,
}

impl Client {
    /// The server and school name comes from the URL to access untis using a browser.
    /// `https://SERVER/WebUntis/jsonrpc.do?school=SCHOOL`
    /// Username and password are required.
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

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn status_data(&mut self) -> Result<StatusData, Error> {
        self.rpc_client.request("getStatusData", ())
    }

    pub fn holidays(&mut self) -> Result<Vec<Holiday>, Error> {
        self.rpc_client.request("getHolidays", ())
    }

    pub fn rooms(&mut self) -> Result<Vec<Room>, Error> {
        self.rpc_client.request("getRooms", ())
    }

    pub fn classes(&mut self) -> Result<Vec<Class>, Error> {
        self.rpc_client.request("getKlassen", ())
    }

    pub fn subjects(&mut self) -> Result<Vec<Subject>, Error> {
        self.rpc_client.request("getSubjects", ())
    }

    pub fn teachers(&mut self) -> Result<Vec<Teacher>, Error> {
        self.rpc_client.request("getTeachers", ())
    }

    pub fn students(&mut self) -> Result<Vec<Student>, Error> {
        self.rpc_client.request("getStudents", ())
    }

    pub fn own_timetable_until(&mut self, end_date: &Date) -> Result<Timetable, Error> {
        self.own_timetable_between(&Date::today(), end_date)
    }

    pub fn own_timetable_current_week(&mut self) -> Result<Timetable, Error> {
        self.own_timetable_for_week(&Date::today())
    }

    pub fn own_timetable_for_week(&mut self, date: &Date) -> Result<Timetable, Error> {
        self.own_timetable_between(&date.relative_week_begin(), &date.relative_week_end())
    }

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

    pub fn timetable_until(
        &mut self,
        id: &usize,
        ty: &ElementType,
        end_date: &Date,
    ) -> Result<Timetable, Error> {
        self.timetable_between(id, ty, &Date::today(), end_date)
    }

    pub fn timetable_current_week(
        &mut self,
        id: &usize,
        ty: &ElementType,
    ) -> Result<Timetable, Error> {
        self.timetable_for_week(id, ty, &Date::today())
    }

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

    pub fn departments(&mut self) -> Result<Departments, Error> {
        self.rpc_client.request("getDepartments", ())
    }

    /// Quits the current session
    pub fn logout(mut self) -> Result<(), Error> {
        self.rpc_client.request("logout", ())
    }
}

impl School {
    pub fn client_login(self, username: &str, password: &str) -> Result<Client, Error> {
        Client::login(&self.server, &self.login_name, username, password)
    }
}

fn make_untis_url(server: &str, school: &str) -> String {
    format!("https://{}/WebUntis/jsonrpc.do?school={}", server, school)
}
