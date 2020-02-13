fn main() {
    let mut game :[[u8; 9]; 9] = [
        [5,3,0,  0,7,0,  0,0,0],
        [6,0,0,  1,9,5,  0,0,0],
        [0,9,8,  0,0,0,  0,6,0],

        [8,0,0,  0,6,0,  0,0,3],
        [4,0,0,  8,0,3,  0,0,1],
        [7,0,0,  0,2,0,  0,0,6],

        [0,6,0,  0,0,0,  2,8,0],
        [0,0,0,  4,1,9,  0,0,5],
        [0,0,0,  0,8,0,  0,7,9]
    ];
    solve_game(&mut game);
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
                        //println!("{:#?}", &game);
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