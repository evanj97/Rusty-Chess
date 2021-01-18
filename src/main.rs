mod board;
mod piece;

use piece::PieceType;
use piece::Piece;
use std::io;
use std::io::Result;

fn main()
{
    let mut board = board::BoardState::new();

    board.reset();
    board.draw_board();

    let help_message =
        "move <origin X> <origin Y> <destination X> <destination Y>\n\
        valid_move <origin X> <origin Y> <destination X> <destination Y>\n\
        draw\n\
        reset\n\
        help";

    loop
    {
        println!(":");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line"); // read input

        loop {
            if input.ends_with('\n') || input.ends_with('\r') || input.ends_with(' ')
            {
                input.pop();
            } else { break; }
        }


        let vector: Vec<&str> = input.split(' ').collect();

        match vector.get(0)
        {
            Some(&"move") => { call_move(&mut board, &vector); },
            Some(&"is_valid_move") => { call_valid_move(&mut board, &vector); },
            Some(&"draw") => { board.draw_board(); },
            Some(&"reset") => {
                board.reset();
                board.draw_board();
            },
            Some(&"help") => { println!("{}", help_message); },
            Some(&"exit") => {
                println!("EXITING");
                return;
            }
            _ => {
                println!("unknown command");
                continue;
            }
        }
    }

    return;
}

fn call_move(bs: &mut board::BoardState, s: &Vec<&str>) -> bool
{
    if s.len() != 5
    {
        for i in s
        {
            println!("{}", i);
        }
    }

    let mut array: [u8; 4] = [0; 4];

    for i in 1..5
    {
        if let Some(slice) = s.get(i)
        {
            if let Ok(val) = slice.parse::<u8>()
            {
                array[i - 1] = val;
            } else {
                println!("usage: move <origin X> <origin Y> <destination X> <destination Y>");
                return false;
            }
        } else {
            println!("usage: move <origin X> <origin Y> <destination X> <destination Y>");
            return false;
        } // if failed to parse term, return false
    }

    return if bs.move_piece(array[0], array[1], array[2], array[3])
    {
        println!("successfully moved");
        true
    } else {
        println!("invalid move");
        false
    }
}

fn call_valid_move(bs: &mut board::BoardState, s: &Vec<&str>) -> bool
{
    if s.len() != 5
    {
        for i in s
        {
            println!("{}", i);
        }
    }

    let mut array: [u8; 4] = [0; 4];

    for i in 1..5
    {
        if let Some(slice) = s.get(i)
        {
            if let Ok(val) = slice.parse::<u8>() // parse str to u8
            {
                array[i - 1] = val;
            } else {
                println!("usage: is_valid_move <origin X> <origin Y> <destination X> <destination Y>");
                return false;
            }
        } else {
            println!("usage: is_valid_move <origin X> <origin Y> <destination X> <destination Y>");
            return false;
        }
    }


    return if bs.is_valid_move(array[0], array[1], array[2], array[3])
    {
        println!("valid move");
        true
    } else {
        println!("invalid move");
        false
    }
}
