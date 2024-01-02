use std::cmp::{max, min};
use std::collections::HashSet;
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

fn find_descendants_that_would_fall(
    prev_next_data: &Vec<(Vec<usize>, Vec<usize>)>,
    would_fall_indices: &mut HashSet<usize>,
    child_index: usize,
) {
    for i in prev_next_data[child_index].1.iter() {
        if prev_next_data[*i]
            .0
            .iter()
            .all(|j| would_fall_indices.contains(j))
        {
            would_fall_indices.insert(*i);
            find_descendants_that_would_fall(prev_next_data, would_fall_indices, *i);
        }
    }
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

    let mut prev_next_data: Vec<(Vec<usize>, Vec<usize>)> = vec![(vec![], vec![]); bricks.len()];

    for i in 0..bricks.len() {
        for j in 0..bricks.len() {
            if is_blocked(bricks[i], bricks[j]) {
                prev_next_data[i].0.push(j);
                prev_next_data[j].1.push(i);
            }
        }
    }

    for i_removed in 0..bricks.len() {
        let mut would_fall_indices: HashSet<usize> = HashSet::new();

        // find children, that would fall
        for i_child in prev_next_data[i_removed].1.iter() {
            if prev_next_data[*i_child].0.len() == 1 {
                would_fall_indices.insert(*i_child);
            }
        }

        // now further descendants, that would fall
        for i_child in would_fall_indices.clone().iter() {
            find_descendants_that_would_fall(&prev_next_data, &mut would_fall_indices, *i_child);
        }

        result += would_fall_indices.len();
    }

    println!("Final answer: {:?}", result);

    Ok(())
}
