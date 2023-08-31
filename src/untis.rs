use reqwest;

use chrono::NaiveDate;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

use super::*;

/// Provides access to webuntis.com
pub struct Units {
    student: String,
    password: String,
    url: String,

    client: reqwest::blocking::Client,
    session: Option<SessionInfo>,
}

impl Units {
    /// The server and school name comes from the URL to access untis using a browser.
    /// `https://SERVER.webuntis.com/WebUntis/jsonrpc.do?school=SCHOOL`
    /// Username and password are required.
    pub fn new(server: &str, school: &str, student: &str, password: &str) -> Self {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        Units {
            student: student.to_owned(),
            password: password.to_owned(),
            url: make_untis_url(server, school),

            client,
            session: None,
        }
    }

    /// Creates a new session
    pub fn login(&mut self) -> Result<SessionInfo, Error> {
        let params = ParamsAuthenticate::new(self.student.clone(), self.password.clone());
        let request = RpcRequest::new("authenticate", params);

        let re = self.get().json(&request).send()?;

        let status = re.status();

        if status.is_success() {
            let resp = re.text()?;
            let response: RpcResponse<SessionInfo> = serde_json::from_str(&resp)?;
            let result = response.result;
            self.session = Some(result.clone());

            Ok(result)
        } else {
            Err(Error::Http(status))
        }
    }

    pub fn session(&self) -> Option<&SessionInfo> {
        self.session.as_ref()
    }

    pub fn status_data(&self) -> Result<StatusData, Error> {
        self.retrieve("getStatusData", ())
    }

    pub fn holidays(&self) -> Result<Holidays, Error> {
        self.retrieve("getHolidays", ())
    }

    pub fn rooms(&self) -> Result<Rooms, Error> {
        self.retrieve("getRooms", ())
    }

    pub fn classes(&self) -> Result<Classes, Error> {
        self.retrieve("getKlassen", ())
    }

    pub fn subjects(&self) -> Result<Subjects, Error> {
        self.retrieve("getSubjects", ())
    }

    pub fn timetable(&self, id: usize, ty: usize, date: NaiveDate) -> Result<Timetable, Error> {
        let params = ParamsTimetable::new(id, ty, date);
        self.retrieve("getTimetable", params)
    }

    pub fn departments(&self) -> Result<Departments, Error> {
        self.retrieve("getDepartments", ())
    }

    /// Quits the current session
    pub fn logout(&mut self) -> Result<(), Error> {
        if self.session.is_none() {
            return Err(Error::NoSession);
        }

        let request = RpcRequest::new("logout", ());

        let re = self.get().json(&request).send()?;

        let status = re.status();

        if status.is_success() {
            Ok(())
        } else {
            Err(Error::Http(status))
        }
    }

    fn get(&self) -> reqwest::blocking::RequestBuilder {
        self.client.get(&self.url)
    }

    fn retrieve<T: DeserializeOwned, P: Serialize>(
        &self,
        method: &'static str,
        params: P,
    ) -> Result<T, Error> {
        if self.session.is_none() {
            return Err(Error::NoSession);
        }

        let request = RpcRequest::new(method, params);

        let re = self.get().json(&request).send()?;

        let status = re.status();

        if status.is_success() {
            let text = re.text()?;
            let rpc_response: RpcResponse<T> = serde_json::from_str(&text)?;
            Ok(rpc_response.result)
        } else {
            Err(Error::Http(status))
        }
    }
}

fn make_untis_url(server: &str, school: &str) -> String {
    format!(
        "https://{}.webuntis.com/WebUntis/jsonrpc.do?school={}",
        server, school
    )
}
