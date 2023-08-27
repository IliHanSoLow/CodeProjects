use std::io;
use termsize;

static mut MAP: Vec<Vec<bool>> = vec![];
fn main() {
    let mut coords: Vec<(u16, u16)> = vec![];
    let mut get_coords = true;

    while get_coords {
        let mut buffer = String::new();
        println!("x_coord:");
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read the line");
        let mut buffer2 = String::new();
        println!("y_coord");
        io::stdin()
            .read_line(&mut buffer2)
            .expect("failed to read the line");
        coords.push((
            buffer
                .trim()
                .parse()
                .expect(&format!("Couldnt parse buffer {}", buffer)),
            buffer2
                .trim()
                .parse()
                .expect(&format!("Couldnt parse buffer2 {}", buffer2)),
        ));

        let mut done = false;
        while !done {
            let mut buffer3 = String::new();
            println!("anoter one? (y/n)");
            io::stdin()
                .read_line(&mut buffer3)
                .expect("failed to read the line");
            match buffer3.trim() {
                "y" => done = true,
                "n" => {
                    get_coords = false;
                    done = true
                }
                _ => println!("only \"y\" or \"n\" accepted"),
            }
        }
    }
    init(coords);
    unsafe {
        gameloop();
    }
}

fn check_term_size() -> (u16, u16) {
    let mut xsize: u16 = 0;
    let mut ysize: u16 = 0;
    termsize::get().map(|size| {
        xsize = size.cols;
        ysize = size.rows;
    });
    (xsize, ysize - 1)
}

fn init(coords: Vec<(u16, u16)>) {
    let (xsize, ysize) = check_term_size();
    let mut tmp_map = vec![vec![false; ysize.into()]; xsize.into()];
    for j in 0..ysize {
        if j == 0 || j == ysize - 1 {
            for i in 0..xsize {
                for x in 0..coords.len() {
                    if (j, i) == coords[x] {
                        tmp_map[j as usize][i as usize] = true;
                    }
                }
            }
        }
    }
    for _i in 0..ysize {
        print!("\n");
    }
    unsafe {
        MAP = tmp_map;
    }
}

unsafe fn get_surroundings(x: usize, y: usize) -> u8 {
    let mut counter = 0;
    for i in x - 1..x + 1 {
        for j in y - 1..y + 1 {
            if MAP[i][j] == true {
                counter += 1;
            }
        }
    }
    counter - 1
}

unsafe fn gameloop() {
    loop {
        let mut future = MAP.clone();
        for i in 0..MAP.len() {
            for j in 0..MAP[0].len() {
                if MAP[i][j] {
                    match get_surroundings(i as usize, j as usize) {
                        0..=1 => future[i][j] = false,
                        3.. => future[i][j] = false,
                        _ => {}
                    }
                } else {
                    if get_surroundings(i, j) == 3 {
                        future[i][j] = true;
                    }
                }
            }
        }
        MAP = future.clone();
        print_grid();
    }
}

unsafe fn print_grid() {
    for i in 0..MAP.len() {
        for j in 0..MAP[0].len() {
            match MAP[i][j] {
                true => print!("#"),
                false => print!(" "),
            }
        }
        print!("\n")
    }
}
