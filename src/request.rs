use serde::Serialize;

use chrono::NaiveDate;

use date::UntisDate;

#[derive(Serialize)]
pub struct RpcRequest<P: Serialize> {
    id: &'static str,
    method: &'static str,
    jsonrpc: &'static str,
    params: P,
}

impl<P: Serialize> RpcRequest<P> {
    pub fn new(method: &'static str, params: P) -> Self {
        RpcRequest {
            id: "ID",
            method,
            jsonrpc: "2.0",
            params,
        }
    }
}

#[derive(Serialize)]
pub struct ParamsAuthenticate {
    user: String,
    client: &'static str,
    password: String,
}

impl ParamsAuthenticate {
    pub fn new(user: String, password: String) -> Self {
        ParamsAuthenticate {
            user,
            client: "untis-rs",
            password,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamsTimetable {
    id: usize,

    #[serde(rename = "type")]
    ty: usize,

    start_date: UntisDate,
    end_date: UntisDate,
}

impl ParamsTimetable {
    pub fn new(id: usize, ty: usize, date: NaiveDate) -> Self {
        ParamsTimetable {
            id,
            ty,
            start_date: UntisDate::week_begin_from(date),
            end_date: UntisDate::week_end_from(date),
        }
    }
}
