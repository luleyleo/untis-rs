use crate::{
    error::Error, jsonrpc, params::FindSchoolParams, resources::School, SchoolSearchResult,
};

fn get_client() -> jsonrpc::Client {
    jsonrpc::Client::new("https://mobile.webuntis.com/ms/schoolquery2")
}

pub fn search_schools(query: &str) -> Result<Vec<School>, Error> {
    let result = get_client().request(
        "searchSchool",
        vec![FindSchoolParams::Search { search: query }],
    );
    catch_err(result)
}

pub fn get_school_by_id(id: &usize) -> Result<School, Error> {
    let result = get_client().request(
        "searchSchool",
        vec![FindSchoolParams::ById { schoolid: id }],
    );

    get_first(catch_err(result)?)
}

pub fn get_school_by_name(name: &str) -> Result<School, Error> {
    let result = get_client().request(
        "searchSchool",
        vec![FindSchoolParams::ByName { schoolname: name }],
    );

    get_first(catch_err(result)?)
}

fn get_first(mut list: Vec<School>) -> Result<School, Error> {
    if list.len() == 0 {
        Err(Error::NotFound)
    } else {
        Ok(list.swap_remove(0))
    }
}

fn catch_err(result: Result<SchoolSearchResult, Error>) -> Result<Vec<School>, Error> {
    match result {
        Ok(v) => Ok(v.schools),
        Err(Error::Rpc(err)) => {
            if err.code == jsonrpc::ErrorCode::TooManyResults.as_isize() {
                Ok(vec![])
            } else {
                Err(Error::Rpc(err))
            }
        }
        Err(err) => Err(err),
    }
}
