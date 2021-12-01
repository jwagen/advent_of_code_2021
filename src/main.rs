mod day1;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let day: u32 = args[1].parse().unwrap();

    let file = std::fs::read(format!("input/{}.txt", day)).expect("The input file does not exist");
    let input = String::from_utf8(file).unwrap();
    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let day_func = match day {
        1 => day1::run,
        _ => panic!("This day is not implemented"),
    };

    day_func(numbers);
}
