use std::env;
use std::path::Path;

mod day_01;
mod day_02;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("Invalid number of arguments");
    }

    match args[1].as_str() {
        "day_01" => {
            let path = Path::new("data/input_01.txt");
            let data = day_01::read_data(path).unwrap();

            let solution = day_01::solve_part_1(&data).unwrap();
            println!("DAY 1 - Solution A: {}", solution);

            let solution = day_01::solve_part_2(&data).unwrap();
            println!("DAY 1 - Solution B: {}", solution);
        },
        "day_02" => {
            let path = Path::new("data/input_02.txt");
            let data = day_02::read_data(path);

            let solution = day_02::solve_part_1(&data);
            println!("DAY 2 - Solution A: {}", solution);

            let solution = day_02::solve_part_2(&data);
            println!("DAY 2 - Solution B: {}", solution);
        }
        _ => panic!("Invalid day"),
    }
}
