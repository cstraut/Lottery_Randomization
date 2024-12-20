extern crate rand;
extern crate rusqlite;

use rand::seq::SliceRandom;
use rusqlite::{params, Connection};

fn insert_draw(conn: &Connection, data_tuple: (i32, i32, i32, i32, i32, i32, i32)) {
    let sql_str = "INSERT INTO draws VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";
    conn.execute(
        sql_str,
        params![
            data_tuple.0,
            data_tuple.1,
            data_tuple.2,
            data_tuple.3,
            data_tuple.4,
            data_tuple.5,
            data_tuple.6
        ],
    )
    .unwrap();
}

fn refresh_draws(conn: &Connection) {
    let sql_str = "DROP TABLE IF EXISTS draws";
    conn.execute(sql_str, params![]).unwrap();

    let sql_str = "CREATE TABLE IF NOT EXISTS draws (id INTEGER PRIMARY KEY, ball_1 INTEGER,
        ball_2 INTEGER, ball_3 INTEGER, ball_4 INTEGER, ball_5 INTEGER, powerball INTEGER)";
    conn.execute(sql_str, params![]).unwrap();
}

fn calculate_weights(balls: &[i32]) -> Vec<f64> {
    let mut weights = vec![];

    let length = balls.len();

    for n in 1..=70 {
        weights.push((balls.iter().filter(|&x| *x == n).count() as f64) / (length as f64));
    }

    weights
}

fn flatten_list(list_of_lists: &[Vec<i32>]) -> Vec<i32> {
    let mut flattened_list = vec![];

    for sublist in list_of_lists {
        for number in sublist {
            flattened_list.push(*number);
        }
    }

    flattened_list
}

fn main() {
    let conn = Connection::open("powerball.db").unwrap();

    let mut white_balls = vec![];
    let mut power_balls = vec![];

    let sql_str = "SELECT ball_1, ball_2, ball_3, ball_4, ball_5 FROM powerball";
    let results: Vec<Vec<i32>> = conn
        .query(sql_str, params![])
        .unwrap()
        .map(|row| {
            vec![
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
            ]
        })
        .collect();

    let sql_str = "SELECT powerball FROM powerball";
    power_balls = conn
        .query(sql_str, params![])
        .unwrap()
        .map(|row| row.get(0).unwrap())
        .collect();

    white_balls = flatten_list(&results);

    let size_white_balls = white_balls.len();
    let size_power_balls = power_balls.len();

    white_balls.sort();
    power_balls.sort();

    println!("Number of white balls - {}", size_white_balls);
    println!("Number of power balls - {}", size_power_balls);

    // Initialize the DRAWS table
    refresh_draws(&conn);

    let mut row_id = 1;

    let mut rng = rand::thread_rng();

    for _ in 0..500000 {
        let mut draw = vec![];

        for _ in 0..5 {
            let index = rng.gen_range(0, size_white_balls);
            let mut pick = white_balls[index];

            while draw.contains(&pick) {
                let index = rng.gen_range(0, size_white_balls);
                pick = white_balls[index];
            }

            draw.push(pick);
        }

        let index = rng.gen_range(0, size_power_balls);
        let pick_power_ball = power_balls[index];

        // Insert data into the database
        draw.sort();
        let insert_tuple = (
            row_id,
            draw[0],
            draw[1],
            draw[2],
            draw[3],
            draw[4],
            pick_power_ball,
        );
        insert_draw(&conn, insert_tuple);

        row_id += 1;
        if row_id % 1000 == 0 {
            println!("Iteration - {}", row_id);
        }
    }
}
