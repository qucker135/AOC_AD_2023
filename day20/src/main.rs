use std::io;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn main() -> io::Result<()> {
    // values retrieved from manual analysis of graph
    // see tree.dot - input.txt converted to graphviz format
    // https://graphviz.org/
    let periods: Vec<i64> = vec![
        4096 - 2 - 1,
        4096 - 64 - 8 - 4 - 1,
        4096 - 256 - 64 - 32 - 8 - 2 - 1,
        4096 - 128 - 32 - 16 - 8 - 1,
    ];

    let result = periods.iter().fold(1, |acc, &x| lcm(acc, x));

    println!("Final answer: {:?}", result);

    Ok(())
}
