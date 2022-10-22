use std::{
    io::{self, Write},
    str::FromStr,
};

fn read<T>(text: String, validation: &dyn Fn(&T) -> bool) -> T
where
    T: FromStr,
{
    loop {
        print!("{}", text);
        io::stdout().flush().unwrap();

        let mut data = String::new();
        io::stdin().read_line(&mut data).unwrap();
        {
            let result = data.trim().parse::<T>();
            if let Ok(t) = result {
                if validation(&t) {
                    return t;
                } else {
                    println!("value outside the board");
                }
            }
        }
    }
}

fn verify<'a>(p0: &'a str, p1: &'a str, p2: &'a str) -> Option<&'a str> {
    if p0 != " " && p0 == p1 && p1 == p2 {
        return Some(p0);
    }
    return None;
}

fn main() -> io::Result<()> {
    let mut current_player = "O";
    let mut game = [[" ", " ", " "], [" ", " ", " "], [" ", " ", " "]];

    fn onboard(v: &usize) -> bool {
        0 < *v && *v < 4
    }

    loop {
        if current_player == "O" {
            current_player = "X";
        } else {
            current_player = "O";
        }

        println!("   1 | 2 | 3  X");
        println!(" 1 {} | {} | {} ", game[0][0], game[0][1], game[0][2]);
        println!("-----+---+---");
        println!(" 2 {} | {} | {} ", game[1][0], game[1][1], game[1][2]);
        println!("-----+---+---");
        println!(" 3 {} | {} | {} ", game[2][0], game[2][1], game[2][2]);
        println!(" Y");

        let results = vec![
            // vertical
            verify(game[0][0], game[0][1], game[0][2]),
            verify(game[1][0], game[1][1], game[1][2]),
            verify(game[2][0], game[2][1], game[2][2]),
            // horizontal
            verify(game[0][0], game[1][0], game[2][0]),
            verify(game[0][1], game[1][1], game[2][1]),
            verify(game[0][2], game[1][2], game[2][2]),
            // diagonal
            verify(game[0][0], game[1][1], game[2][2]),
            verify(game[2][0], game[1][1], game[0][2]),
        ];
        // verify
        let mut endgame = false;
        for r in results.iter() {
            if let Some(winner) = r {
                println!("============== END GAME ==============");
                println!("Player {} Won!", winner);
                endgame = true;
                break;
            }
        }
        if !endgame && game.iter().all(|x| x.iter().all(|y| *y != " ")) {
            println!("============== DRAW GAME ==============");
            endgame = true;
        }

        if endgame {
            break;
        }

        // read players input
        loop {
            let x: usize = read(format!("Player {}, X", current_player), &onboard);
            let y: usize = read(format!("Player {}, Y", current_player), &onboard);

            if game[y - 1][x - 1] == " " {
                game[y - 1][x - 1] = current_player;
                break;
            } else {
                println!("position already taken, please choose another")
            }
        }
    }

    Ok(())
}
