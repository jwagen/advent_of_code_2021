pub fn run(input: Vec<u32>) {
    // Open file

    let val = |n| {
        input
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
