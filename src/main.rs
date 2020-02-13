use std::io;


fn main() {
    let mut game: [[u8; 9]; 9] = get_grid();
    print!("\n\n"); // Delete if you dont want space between input and output
    solve_game(&mut game);
    let mut wait = String::new();  // Delete if you dont want to wait for user input before closing program
    io::stdin().read_line(&mut wait); //
}


fn check_placement(grid: &[[u8; 9]; 9], location: (usize,usize), number: u8) -> bool{
    for i in 0..9{
        if grid[location.0][i] == number{
            return false;
        }
    }
    for i in 0..9{
        if grid[i][location.1] == number{
            return false;
        }
    }
    let x_block_start = location.1 / 3 * 3;
    let y_block_start = location.0 / 3 * 3;
    for i in 0..3{
        for j in 0..3{
            if grid[y_block_start + i][x_block_start + j] == number{
                return false;
            }
        }
    }
    true
}


fn solve_game(game: &mut [[u8; 9]; 9]) -> bool{
    for y in 0..9{
        for x in 0..9{
            if game[y][x] == 0{
                for n in 1..10{
                    if check_placement(&game, (y,x), n){
                        game[y][x] = n;
                        solve_game(game);
                        game[y][x] = 0;
                    }
                }
                return false;
            }
        }
    }
    pretty_print_grid(game);
    return true;
}


fn pretty_print_grid(grid: &[[u8; 9]; 9]){
    for (j, line) in grid.iter().enumerate(){
        print!(" ");
        for (i, number) in line.iter().enumerate(){
            print!("{}", number);
            if (i + 1) % 3 == 0{
                print!(" ");
            }
        }
        println!("");
        if (j + 1) % 3 == 0{
            println!("");
        }
    }
}


fn get_grid() -> [[u8;9];9]{
    let mut result: [[u8;9];9] = [[0;9];9];
    for i in 0..9{
        let mut numbers = String::new();
        io::stdin().read_line(&mut numbers).ok().expect("read error");
        let mut numbers: Vec<u8> = numbers
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut temp: [u8; 9] = [0;9];
        temp.copy_from_slice(&mut numbers[..9]);
        result[i] = temp;    
    }
    return result;
}
