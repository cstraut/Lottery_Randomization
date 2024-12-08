use postgres::{Client, Error, NoTls};
use rand::prelude::*;

fn main() -> Result<(), Error> {
    let database_url = "postgres://postgres:password@172.17.0.2:5432/Powerball_db".to_string();

    // Create vector for white and power balls
    let mut pb_white_balls: Vec<i32> = vec![];
    let mut pb_money_balls: Vec<i32> = vec![];
    let mut mm_white_balls: Vec<i32> = vec![];
    let mut mm_money_balls: Vec<i32> = vec![];

    let mut client = Client::connect(&database_url, NoTls)?;

    // Drop the previous draws table and create a new one
    client.batch_execute("DROP TABLE IF EXISTS public.\"DRAWS_SS_PB\";")?;
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS public.\"DRAWS_SS_PB\" (
                DRAW_ID SERIAL PRIMARY KEY,
                BALL_1 INTEGER NOT NULL,
                BALL_2 INTEGER NOT NULL,
                BALL_3 INTEGER NOT NULL,
                BALL_4 INTEGER NOT NULL,
                BALL_5 INTEGER NOT NULL,
                MONEYBALL INTEGER NOT NULL);",
    )?;

    println!("Powerball Small sample set Draws table created");

    // Drop the previous draws table and create a new one
    client.batch_execute("DROP TABLE IF EXISTS public.\"DRAWS_SS_MM\";")?;
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS public.\"DRAWS_SS_MM\" (
                DRAW_ID SERIAL PRIMARY KEY,
                BALL_1 INTEGER NOT NULL,
                BALL_2 INTEGER NOT NULL,
                BALL_3 INTEGER NOT NULL,
                BALL_4 INTEGER NOT NULL,
                BALL_5 INTEGER NOT NULL,
                MONEYBALL INTEGER NOT NULL);",
    )?;

    println!("Mega Millions Samll sample set Draws table created");

    for row in client.query(
        "SELECT BALL_1, BALL_2, BALL_3, BALL_4, BALL_5, MONEYBALL FROM public.\"POWERBALL\" WHERE DRAW_DATE > 20240109;",
        &[],
    )? {
        pb_white_balls.push(row.get(0));
        pb_white_balls.push(row.get(1));
        pb_white_balls.push(row.get(2));
        pb_white_balls.push(row.get(3));
        pb_white_balls.push(row.get(4));
        pb_money_balls.push(row.get(5));
    }

    println!("Powerball data fetched");

    for row in client.query(
        "SELECT BALL_1, BALL_2, BALL_3, BALL_4, BALL_5, MONEYBALL FROM public.\"MEGAMILLIONS\" WHERE DRAW_DATE > 20230914;",
        &[],
    )? {
        mm_white_balls.push(row.get(0));
        mm_white_balls.push(row.get(1));
        mm_white_balls.push(row.get(2));
        mm_white_balls.push(row.get(3));
        mm_white_balls.push(row.get(4));
        mm_money_balls.push(row.get(5));
    }

    println!("MegaMillions data fetched");

    let pb_white_balls_size = pb_white_balls.len();
    let pb_money_balls_size = pb_money_balls.len();
    let mm_white_balls_size = mm_white_balls.len();
    let mm_money_balls_size = mm_money_balls.len();

    // Print size of white balls and power balls vectors
    println!(
        "Powerball - White balls - {:?}\tMoney balls - {:?}",
        pb_white_balls_size, pb_money_balls_size
    );

    // Print size of white balls and power balls vectors
    println!(
        "MegaMillions - White balls - {:?}\tMoney balls - {:?}",
        mm_white_balls_size, mm_money_balls_size
    );

    pb_white_balls.sort();
    pb_money_balls.sort();

    mm_white_balls.sort();
    mm_money_balls.sort();

    let mut rng = thread_rng();
    let mut draw: Vec<i32> = vec![];
    let mut index_key: u32;
    let mut pick: i32;

    println!("Starting Powerball draws!");
    for idx in 0..2000000 {
        for n in 0..5 {
            index_key = rng.gen_range(0..pb_white_balls_size as u32);
            pick = pb_white_balls[index_key as usize];

            if n == 0 {
                draw.push(pick);
            } else {
                if draw.contains(&pick) {
                    while draw.contains(&pick) {
                        index_key = rng.gen_range(0..pb_white_balls_size as u32);
                        pick = pb_white_balls[index_key as usize];
                    }
                }

                draw.push(pick);
            }
        }

        draw.sort();

        index_key = rng.gen_range(0..pb_money_balls_size as u32);
        draw.push(pb_money_balls[index_key as usize]);

        // Insert the draw vector into the database
        let insert_tuple = (draw[0], draw[1], draw[2], draw[3], draw[4], draw[5]);

        client.execute(
            "INSERT INTO public.\"DRAWS_SS_PB\" (BALL_1, BALL_2, BALL_3, BALL_4, BALL_5, MONEYBALL)
            VALUES ($1, $2, $3, $4, $5, $6);",
            &[
                &insert_tuple.0,
                &insert_tuple.1,
                &insert_tuple.2,
                &insert_tuple.3,
                &insert_tuple.4,
                &insert_tuple.5,
            ],
        )?;

        draw.clear();

        if idx > 0 && idx % 10000 == 0 {
            println!("Inserted {} Powerball draws", idx);
        }
    }

    println!("Starting Mega Millions draws!");
    for idx in 0..3000000 {
        for n in 0..5 {
            index_key = rng.gen_range(0..mm_white_balls_size as u32);
            pick = mm_white_balls[index_key as usize];

            if n == 0 {
                draw.push(pick);
            } else {
                if draw.contains(&pick) {
                    while draw.contains(&pick) {
                        index_key = rng.gen_range(0..mm_white_balls_size as u32);
                        pick = mm_white_balls[index_key as usize];
                    }
                }

                draw.push(pick);
            }
        }

        draw.sort();

        index_key = rng.gen_range(0..mm_money_balls_size as u32);
        draw.push(mm_money_balls[index_key as usize]);

        // Insert the draw vector into the database
        let insert_tuple = (draw[0], draw[1], draw[2], draw[3], draw[4], draw[5]);

        client.execute(
            "INSERT INTO public.\"DRAWS_SS_MM\" (BALL_1, BALL_2, BALL_3, BALL_4, BALL_5, MONEYBALL)
            VALUES ($1, $2, $3, $4, $5, $6);",
            &[
                &insert_tuple.0,
                &insert_tuple.1,
                &insert_tuple.2,
                &insert_tuple.3,
                &insert_tuple.4,
                &insert_tuple.5,
            ],
        )?;

        draw.clear();

        if idx > 0 && idx % 10000 == 0 {
            println!("Inserted {} MegaMillions draws", idx);
        }
    }

    Ok(())
}
