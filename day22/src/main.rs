use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vertex {
    x: i64,
    y: i64,
    z: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Brick(Vertex, Vertex);

fn is_blocked(blocked: Brick, blocking: Brick) -> bool {
    max(blocking.0.z, blocking.1.z) + 1 == min(blocked.0.z, blocked.1.z)
        && !(max(blocked.0.x, blocked.1.x) < min(blocking.0.x, blocking.1.x)
            || min(blocked.0.x, blocked.1.x) > max(blocking.0.x, blocking.1.x))
        && !(max(blocked.0.y, blocked.1.y) < min(blocking.0.y, blocking.1.y)
            || min(blocked.0.y, blocked.1.y) > max(blocking.0.y, blocking.1.y))
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    let mut bricks: Vec<Brick> = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        let mut vertices: Vec<Vertex> = Vec::new();
        line.split('~').for_each(|vertex_data| {
            let mut coords: Vec<&str> = Vec::new();
            vertex_data.split(',').for_each(|coord| {
                coords.push(coord);
            });
            vertices.push(Vertex {
                x: coords[0].parse::<i64>().unwrap(),
                y: coords[1].parse::<i64>().unwrap(),
                z: coords[2].parse::<u64>().unwrap(),
            });
        });
        bricks.push(Brick(vertices[0], vertices[1]));
    }

    let mut has_fallen = true;

    while has_fallen {
        has_fallen = false;
        for i in 0..bricks.len() {
            let mut can_fall = min(bricks[i].0.z, bricks[i].1.z) > 1;
            if can_fall {
                for brick_blocking in bricks.iter() {
                    if is_blocked(bricks[i], *brick_blocking) {
                        can_fall = false;
                        break;
                    }
                }
            }
            if can_fall {
                has_fallen = true;
                bricks[i].0.z -= 1;
                bricks[i].1.z -= 1;
            }
        }
    }

    // find nr of bricks, that are not the only blocking for all of their blocked bricks
    // if exists a brick that a brick_checked is the only blocking for, then do not count brick_checked
    for brick_checked in bricks.iter() {
        let mut count_checked = true;

        for brick_blocked in bricks.iter() {
            if is_blocked(*brick_blocked, *brick_checked) {
                let mut is_only_blocking = true;
                for brick_blocking in bricks.iter() {
                    if brick_blocking != brick_checked
                        && is_blocked(*brick_blocked, *brick_blocking)
                    {
                        is_only_blocking = false;
                        break;
                    }
                }
                if is_only_blocking {
                    count_checked = false;
                    break;
                }
            }
        }

        if count_checked {
            result += 1;
        }
    }

    println!("Final answer: {:?}", result);

    Ok(())
}
