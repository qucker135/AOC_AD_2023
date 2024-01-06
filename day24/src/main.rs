use ndarray::prelude::*;
use ndarray_linalg::Solve;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Trajectory {
    px: i128,
    py: i128,
    pz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

fn get_coeffs_4_dim(a: Trajectory, b: Trajectory, iz: bool) -> ([i128; 4], i128) {
    if !iz {
        return (
            [a.py - b.py, b.vy - a.vy, b.px - a.px, a.vx - b.vx],
            b.vy * b.px - b.py * b.vx - a.vy * a.px + a.py * a.vx,
        );
    }
    (
        [a.pz - b.pz, b.vz - a.vz, b.px - a.px, a.vx - b.vx],
        b.vz * b.px - b.pz * b.vx - a.vz * a.px + a.pz * a.vx,
    )
}

fn solve_vs_helper(trajectories: &[Trajectory], iz: bool) -> (i128, i128) {
    let mut a: Vec<[i128; 4]> = vec![];
    let mut b: Vec<i128> = vec![];

    for i in 1..=4 {
        let (a_row, b_scalar) =
            get_coeffs_4_dim(trajectories[2 * i - 2], trajectories[2 * i - 1], iz);
        a.push(a_row);
        b.push(b_scalar);
    }

    let a_f: Vec<[f64; 4]> = a
        .iter()
        .map(|x| [x[0] as f64, x[1] as f64, x[2] as f64, x[3] as f64])
        .collect();
    let b_f: Vec<f64> = b.iter().map(|x| *x as f64).collect();

    let a_r: Array2<f64> = array![a_f[0], a_f[1], a_f[2], a_f[3]];
    let b_r: Array1<f64> = array![b_f[0], b_f[1], b_f[2], b_f[3]];

    let x = a_r.solve_into(b_r).unwrap();

    let x = x.iter().map(|x| x.round() as i128).collect::<Vec<_>>();

    (x[0], x[2])
}

fn solve_ps_helper(
    trajectories: &[Trajectory],
    (vx, vy, vz): (i128, i128, i128),
) -> (i128, i128, i128) {
    let a = [
        [
            vx - trajectories[5].vx,
            trajectories[5].vy - vy,
            trajectories[5].vz - vz,
        ],
        [
            vx - trajectories[6].vx,
            trajectories[6].vy - vy,
            trajectories[6].vz - vz,
        ],
    ];
    let b = [
        trajectories[5].px * (trajectories[5].vy - vy)
            + trajectories[5].py * (vx - trajectories[5].vx),
        trajectories[6].px * (trajectories[6].vy - vy)
            + trajectories[6].py * (vx - trajectories[6].vx),
        trajectories[5].px * (trajectories[5].vz - vz)
            + trajectories[5].pz * (vx - trajectories[5].vx),
        trajectories[6].px * (trajectories[6].vz - vz)
            + trajectories[6].pz * (vx - trajectories[6].vx),
    ];
    let wxy = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    let wx = b[0] * a[1][1] - b[1] * a[0][1];
    let wy = a[0][0] * b[1] - a[1][0] * b[0];

    let wxz = a[0][0] * a[1][2] - a[0][2] * a[1][0];
    let wz = b[2] * a[1][2] - b[3] * a[0][2];

    let x = wx / wxy;
    let y = wy / wxy;
    let z = wz / wxz;
    (x, y, z)
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

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

    let (_, vy) = solve_vs_helper(&trajectories, false);
    let (vx, vz) = solve_vs_helper(&trajectories, true);

    println!("vx = {}, vy = {}, vz = {}", vx, vy, vz);

    let (px, py, pz) = solve_ps_helper(&trajectories, (vx, vy, vz));

    println!("px = {}, py = {}, pz = {}", px, py, pz);

    let result = px + py + pz;

    println!("Final answer: {:?}", result);

    Ok(())
}
