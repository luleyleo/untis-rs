# untis-rs

Library for accessing the [Untis](https://www.untis.at) JSON-RPC API.

## API

This client uses the public Untis JSON-RPC API, which only has read-only, limited access.

## Examples

```rust
fn main() -> Result<(), untis::Error> {
  let results = untis::schools::search("School Name")?;
  let school = match results.first() {
    None => {
      println!("No school found");
      return Ok(());
    },
    Some(v) => v
  };

  let mut client = school.client_login("username", "password")?;

  let timetable = client.own_timetable_current_week()?;

  // profit

  client.logout()?;
  Ok(())
}
```

For more examples, see the `examples/` directory.
