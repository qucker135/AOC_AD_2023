use std::cmp::max;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::iter::zip;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn area(points: &[(i64, i64)]) -> i64 {
    let mut res = 0i64;

    for (p1, p2) in zip(
        points.iter(),
        points.iter().cycle().skip(1).take(points.len()),
    ) {
        res += p1.0 * p2.1 - p2.0 * p1.1;
    }

    res.abs() / 2
}

fn max_area(points1: &[(i64, i64)], points2: &[(i64, i64)]) -> i64 {
    max(area(points1), area(points2))
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut plan: Vec<(Direction, i64)> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        let color: &str = &line.split(' ').nth(2).unwrap()[2..8];

        let num = i64::from_str_radix(&color[..5], 16).unwrap();

        match color.chars().last().unwrap() {
            '0' => plan.push((Direction::Right, num)),
            '1' => plan.push((Direction::Down, num)),
            '2' => plan.push((Direction::Left, num)),
            '3' => plan.push((Direction::Up, num)),
            _ => {}
        }
    }

    type PointsList = Vec<(i64, i64)>;

    let (mut points_left, mut points_right): (PointsList, PointsList) = (vec![], vec![]);

    let (mut cur_x_left, mut cur_y_left, mut cur_x_right, mut cur_y_right) =
        (0i64, 0i64, 0i64, 0i64);

    for i in 0..plan.len() {
        let i_prev = (i + plan.len() - 1) % plan.len();
        let i_next = (i + 1) % plan.len();

        if plan[i_prev].0 == plan[i_next].0 {
            match plan[i].0 {
                Direction::Left => {
                    cur_x_left -= plan[i].1;
                    cur_x_right -= plan[i].1;
                }
                Direction::Right => {
                    cur_x_left += plan[i].1;
                    cur_x_right += plan[i].1;
                }
                Direction::Up => {
                    cur_y_left += plan[i].1;
                    cur_y_right += plan[i].1;
                }
                Direction::Down => {
                    cur_y_left -= plan[i].1;
                    cur_y_right -= plan[i].1;
                }
            }
        } else {
            // left inner
            if plan[i_prev].0 == Direction::Down
                && plan[i].0 == Direction::Right
                && plan[i_next].0 == Direction::Up
            {
                cur_x_left += plan[i].1 - 1;
                cur_x_right += plan[i].1 + 1;
            } else if plan[i_prev].0 == Direction::Up
                && plan[i].0 == Direction::Left
                && plan[i_next].0 == Direction::Down
            {
                cur_x_left -= plan[i].1 - 1;
                cur_x_right -= plan[i].1 + 1;
            } else if plan[i_prev].0 == Direction::Right
                && plan[i].0 == Direction::Up
                && plan[i_next].0 == Direction::Left
            {
                cur_y_left += plan[i].1 - 1;
                cur_y_right += plan[i].1 + 1;
            } else if plan[i_prev].0 == Direction::Left
                && plan[i].0 == Direction::Down
                && plan[i_next].0 == Direction::Right
            {
                cur_y_left -= plan[i].1 - 1;
                cur_y_right -= plan[i].1 + 1;
            }
            // right inner
            else if plan[i_prev].0 == Direction::Down
                && plan[i].0 == Direction::Left
                && plan[i_next].0 == Direction::Up
            {
                cur_x_left -= plan[i].1 + 1;
                cur_x_right -= plan[i].1 - 1;
            } else if plan[i_prev].0 == Direction::Up
                && plan[i].0 == Direction::Right
                && plan[i_next].0 == Direction::Down
            {
                cur_x_left += plan[i].1 + 1;
                cur_x_right += plan[i].1 - 1;
            } else if plan[i_prev].0 == Direction::Right
                && plan[i].0 == Direction::Down
                && plan[i_next].0 == Direction::Left
            {
                cur_y_left -= plan[i].1 + 1;
                cur_y_right -= plan[i].1 - 1;
            } else if plan[i_prev].0 == Direction::Left
                && plan[i].0 == Direction::Up
                && plan[i_next].0 == Direction::Right
            {
                cur_y_left += plan[i].1 + 1;
                cur_y_right += plan[i].1 - 1;
            }
        }
        points_left.push((cur_x_left, cur_y_left));
        points_right.push((cur_x_right, cur_y_right));
    }

    let result: i64 = max_area(&points_left, &points_right);

    println!("Final answer: {:?}", result);

    Ok(())
}
