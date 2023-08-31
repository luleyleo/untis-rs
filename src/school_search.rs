use crate::{error::Error, jsonrpc, params::FindSchool, resources::School};

fn get_client() -> jsonrpc::Client {
    jsonrpc::Client::new(String::from("https://mobile.webuntis.com/ms/schoolquery2"))
}

pub fn search_schools(query: &str) -> Result<Vec<School>, Error> {
    get_client()
        .request("searchSchool", vec![FindSchool::Search { search: query }])
        .catch_error_too_many()
}

pub fn get_school_by_id(id: usize) -> Result<School, Error> {
    get_client()
        .request("searchSchool", vec![FindSchool::ById { schoolid: id }])
        .catch_error_too_many()
        .select_first_result()
}

pub fn get_school_by_name(name: &str) -> Result<School, Error> {
    get_client()
        .request(
            "searchSchool",
            vec![FindSchool::ByName { schoolname: name }],
        )
        .catch_error_too_many()
        .select_first_result()
}

trait SearchResult {
    fn catch_error_too_many(self) -> Result<Vec<School>, Error>;
    fn select_first_result(self) -> Result<School, Error>;
}

impl SearchResult for Result<Vec<School>, Error> {
    fn catch_error_too_many(self) -> Result<Vec<School>, Error> {
        if self.is_ok() {
            return self;
        }

        match self.unwrap_err() {
            Error::Rpc(err) => {
                if err.code == jsonrpc::ErrorCode::TooManyResults.value() {
                    Ok(Vec::new())
                } else {
                    Err(Error::Rpc(err))
                }
            }
            err => Err(err),
        }
    }

    fn select_first_result(self) -> Result<School, Error> {
        match self {
            Err(err) => Err(err),
            Ok(v) => match v.first() {
                None => Err(Error::NotFound),
                Some(v) => Ok(v.clone()),
            },
        }
    }
}
