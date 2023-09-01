use chrono;
use untis;

///
/// This example shows a basic usecase of searching for a specific school
/// and then retrieving a user's own timetable.
///

fn main() -> Result<(), untis::Error> {
    // Get the school by its id.
    let school = untis::schools::get_by_id(&42)?;

    // Log in with your credentials. The school's details are filled in automatically.
    let result = school.client_login("username", "password");
    let mut client: untis::Client;

    // Match the result to handle specific error cases.
    match result {
        Ok(v) => client = v,
        Err(untis::Error::Rpc(err)) => {
            if err.code == untis::jsonrpc::ErrorCode::InvalidCredentials.as_isize() {
                println!("Invalid credentials");
            }
            return Err(untis::Error::Rpc(err));
        }
        Err(err) => return Err(err)?,
    };

    let date = chrono::Local::now().date_naive() + chrono::Duration::weeks(2);

    // Get the client's own timetable until 2 weeks from now.
    let timetable = client.own_timetable_until(&untis::Date(date))?;

    for lesson in timetable {
        println!("{:?}", lesson);
    }

    client.logout()?;

    Ok(())
}
