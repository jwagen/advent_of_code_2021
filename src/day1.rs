pub fn run(input: String) {
    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let val = |n| {
        numbers
            .windows(n)
            .map(|v| v.iter().sum::<u32>())
            .collect::<Vec<u32>>()
            .windows(2)
            .filter(|v| v[0] < v[1])
            .count()
    };
    println!("Increasing samples window sise 1: {:?}", val(1));
    println!("Increasing samples window sise 3: {:?}", val(3));
}
