use untis;

///
/// This example shows a basic usecase of searching for schools.
///

fn main() -> Result<(), untis::Error> {
    let schools = untis::schools::search("query")?;

    for school in schools {
        println!("name: {}, server: {}", school.display_name, school.server);
    }

    Ok(())
}
