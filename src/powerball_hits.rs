#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

fn main() {
    // Establish a connection to the SQLite database
    let connection = establish_connection();

    // Define the SQL query
    let query = diesel::sql_query("
        SELECT ball_1, ball_2, ball_3, ball_4, ball_5, powerball
        FROM powerball as pb
        WHERE EXISTS (
            SELECT 1
            FROM draws_1 as d
            WHERE pb.ball_1 = d.ball_1 and
            pb.ball_2 = d.ball_2 and
            pb.ball_3 = d.ball_3 and
            pb.ball_4 = d.ball_4 and
            pb.ball_5 = d.ball_5 and
            pb.powerball = d.powerball
        )
    ");

    // Execute the query
    let results = query.load::<Powerball>(&connection).expect("Error executing query");

    // Display the results
    for result in results {
        // Assuming your Powerball struct has fields `ball_1`, `ball_2`, `ball_3`, `ball_4`, `ball_5`, and `powerball`
        println!(
            "Ball 1: {}, Ball 2: {}, Ball 3: {}, Ball 4: {}, Ball 5: {}, Powerball: {}",
            result.ball_1, result.ball_2, result.ball_3, result.ball_4, result.ball_5, result.powerball
        );
    }
}

// A struct to represent the Powerball data
#[derive(Queryable)]
struct Powerball {
    ball_1: i32,
    ball_2: i32,
    ball_3: i32,
    ball_4: i32,
    ball_5: i32,
    powerball: i32,
}

// Establish a connection to the SQLite database
fn establish_connection() -> SqliteConnection {
    let database_url = "path/to/database.sqlite"; // Replace with your SQLite database URL
    SqliteConnection::establish(&database_url).expect("Failed to connect to database")
}

