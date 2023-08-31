use chrono::NaiveDate;
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::*;

/// Provides access to webuntis.com
pub struct Client {
    client_name: String,
    username: String,
    password: String,
    rpc_client: jsonrpc::Client,
    session: Option<Session>,
}

impl Client {
    /// The server and school name comes from the URL to access untis using a browser.
    /// `https://SERVER.webuntis.com/WebUntis/jsonrpc.do?school=SCHOOL`
    /// Username and password are required.
    pub fn new(server: &str, school: &str, username: &str, password: &str) -> Self {
        Self {
            client_name: String::from("untis-rs"),
            username: username.to_string(),
            password: password.to_string(),
            rpc_client: jsonrpc::Client::new(make_untis_url(server, school)),
            session: None,
        }
    }

    fn request<T: DeserializeOwned, P: Serialize>(
        &mut self,
        method: &'static str,
        params: P,
    ) -> Result<T, Error> {
        if self.session.is_none() {
            return Err(Error::NoSession);
        }

        self.rpc_client.request(method, params)
    }

    /// Creates a new session
    pub fn login(&mut self) -> Result<Session, Error> {
        let params = params::Authenticate::new(&self.client_name, &self.username, &self.password);
        let session: Session = self.rpc_client.request("authenticate", params)?;
        self.session = Some(session.clone());
        Ok(session)
    }

    pub fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    pub fn status_data(&mut self) -> Result<StatusData, Error> {
        self.request("getStatusData", ())
    }

    pub fn holidays(&mut self) -> Result<Vec<Holiday>, Error> {
        self.request("getHolidays", ())
    }

    pub fn rooms(&mut self) -> Result<Vec<Room>, Error> {
        self.request("getRooms", ())
    }

    pub fn classes(&mut self) -> Result<Vec<Class>, Error> {
        self.request("getKlassen", ())
    }

    pub fn subjects(&mut self) -> Result<Vec<Subject>, Error> {
        self.request("getSubjects", ())
    }

    pub fn teachers(&mut self) -> Result<Vec<Teacher>, Error> {
        self.request("getTeachers", ())
    }

    pub fn timetable(
        &mut self,
        id: usize,
        ty: ElementType,
        date: NaiveDate,
    ) -> Result<Timetable, Error> {
        let params = params::Timetable::new(id, ty, date);
        self.request("getTimetable", params)
    }

    pub fn departments(&mut self) -> Result<Departments, Error> {
        self.request("getDepartments", ())
    }

    /// Quits the current session
    pub fn logout(mut self) -> Result<(), Error> {
        self.request("logout", ())
    }
}

fn make_untis_url(server: &str, school: &str) -> String {
    format!("https://{}/WebUntis/jsonrpc.do?school={}", server, school)
}
