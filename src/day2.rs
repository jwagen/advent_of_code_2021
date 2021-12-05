pub fn run(input: String) {
    part1(input.clone());
    part2(input);
}

fn part1(input: String) {
    let mut x = 0;
    let mut y = 0;
    for line in input.split('\n').filter(|l| !l.is_empty()) {
        //println!("{:?}", line);
        let mut line = line.split_whitespace();
        let dir = line.next().unwrap();
        let val: u32 = line.next().unwrap().parse().unwrap();

        match dir {
            "up" => y -= val,
            "down" => y += val,
            "forward" => x += val,
            _ => panic!("Unknown direction"),
        };
    }
    println!("x: {}, y: {}, x*y: {}", x, y, x * y);
}

fn part2(input: String) {
    let mut x = 0_i64;
    let mut y = 0_i64;
    let mut aim = 0_i64;

    for line in input.split('\n').filter(|l| !l.is_empty()) {
        println!("{:?}", line);
        let mut line = line.split_whitespace();
        let dir = line.next().unwrap();
        let val: i64 = line.next().unwrap().parse().unwrap();

        match dir {
            "up" => {
                aim -= val;
            }
            "down" => {
                aim += val;
            }
            "forward" => {
                x += val;
                y += aim*val;
            }
            _ => panic!("Unknown direction"),
        };
        println!("x: {}, y: {}, aim: {}", x, y, aim);
    }
    println!("x: {}, y: {}, x*y: {}", x, y, x * y);
}
