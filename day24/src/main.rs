use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const MAX: i128 = 400_000_000_000_000;
const MIN: i128 = 200_000_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Trajectory {
    px: i128,
    py: i128,
    pz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

fn check_collision(a: Trajectory, b: Trajectory) -> bool {
    let w: i128 = b.vx * a.vy - a.vx * b.vy;
    // check if the two trajectories are parallel
    if w == 0 {
        return false;
    }
    let wa: i128 = b.vx * (b.py - a.py) - b.vy * (b.px - a.px);
    let wb: i128 = a.vx * (b.py - a.py) - a.vy * (b.px - a.px);

    // check if collision would happen in the future
    if wa.signum() != w.signum() || wb.signum() != w.signum() {
        return false;
    }

    // check if collision would happen in considered area
    w.abs() * (MIN - a.px) <= a.vx * wa.abs()
        && a.vx * wa.abs() <= w.abs() * (MAX - a.px)
        && w.abs() * (MIN - a.py) <= a.vy * wa.abs()
        && a.vy * wa.abs() <= w.abs() * (MAX - a.py)
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result: i128 = 0;

    let mut trajectories: Vec<Trajectory> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        let mut bfr: Vec<i128> = vec![];
        line.split(" @ ").for_each(|part: &str| {
            part.split(", ")
                .for_each(|raw_num: &str| bfr.push(raw_num.parse::<i128>().unwrap()))
        });
        trajectories.push(Trajectory {
            px: bfr[0],
            py: bfr[1],
            pz: bfr[2],
            vx: bfr[3],
            vy: bfr[4],
            vz: bfr[5],
        });
    }

    for i in 0..trajectories.len() {
        for j in i + 1..trajectories.len() {
            if check_collision(trajectories[i], trajectories[j]) {
                result += 1;
            }
        }
    }

    println!("Final answer: {:?}", result);

    Ok(())
}
