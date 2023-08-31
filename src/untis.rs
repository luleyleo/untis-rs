use chrono::NaiveDate;

use super::*;
use crate::jsonrpc;

/// Provides access to webuntis.com
pub struct Units {
    client_name: String,
    username: String,
    password: String,
    rpc_client: jsonrpc::Client,
    session: Option<SessionInfo>,
}

impl Units {
    /// The server and school name comes from the URL to access untis using a browser.
    /// `https://SERVER.webuntis.com/WebUntis/jsonrpc.do?school=SCHOOL`
    /// Username and password are required.
    pub fn new(server: &str, school: &str, username: &str, password: &str) -> Self {
        Self::new_with_client_name("untis-rs", server, school, username, password)
    }

    pub fn new_with_client_name(
        client_name: &str,
        server: &str,
        school: &str,
        username: &str,
        password: &str,
    ) -> Self {
        Self {
            client_name: client_name.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            rpc_client: jsonrpc::Client::new(make_untis_url(server, school)),
            session: None,
        }
    }

    /// Creates a new session
    pub fn login(&mut self) -> Result<SessionInfo, Error> {
        let params = ParamsAuthenticate::new(&self.client_name, &self.username, &self.password);

        let session = self
            .rpc_client
            .request::<SessionInfo, ParamsAuthenticate>("authenticate", params)?;

        self.session = Some(session.clone());
        Ok(session)
    }

    pub fn session(&self) -> Option<&SessionInfo> {
        self.session.as_ref()
    }

    pub fn status_data(&mut self) -> Result<StatusData, Error> {
        self.rpc_client.request("getStatusData", ())
    }

    pub fn holidays(&mut self) -> Result<Holidays, Error> {
        self.rpc_client.request("getHolidays", ())
    }

    pub fn rooms(&mut self) -> Result<Rooms, Error> {
        self.rpc_client.request("getRooms", ())
    }

    pub fn classes(&mut self) -> Result<Classes, Error> {
        self.rpc_client.request("getKlassen", ())
    }

    pub fn subjects(&mut self) -> Result<Subjects, Error> {
        self.rpc_client.request("getSubjects", ())
    }

    pub fn timetable(&mut self, id: usize, ty: usize, date: NaiveDate) -> Result<Timetable, Error> {
        let params = ParamsTimetable::new(id, ty, date);
        self.rpc_client.request("getTimetable", params)
    }

    pub fn departments(&mut self) -> Result<Departments, Error> {
        self.rpc_client.request("getDepartments", ())
    }

    /// Quits the current session
    pub fn logout(&mut self) -> Result<(), Error> {
        if self.session.is_none() {
            return Err(Error::NoSession);
        }

        self.rpc_client.request("logout", ())
    }
}

fn make_untis_url(server: &str, school: &str) -> String {
    format!(
        "https://{}.webuntis.com/WebUntis/jsonrpc.do?school={}",
        server, school
    )
}
