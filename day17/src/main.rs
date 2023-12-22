use core::{cmp::Eq, cmp::Ord, hash::Hash, ops::Neg};
use infinitable::*;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
enum LastMove {
    Horizontal,
    Vertical,
}

struct MinPriorityQueue<I, P>
where
    I: Hash + Eq,
    P: Ord,
{
    pq: PriorityQueue<I, P>,
}

impl<I, P> MinPriorityQueue<I, P>
where
    I: Hash + Eq,
    P: Ord + Neg<Output = P>,
{
    fn new() -> MinPriorityQueue<I, P> {
        MinPriorityQueue {
            pq: PriorityQueue::new(),
        }
    }

    fn push(&mut self, item: I, priority: P) {
        self.pq.push(item, -priority);
    }

    fn change_priority(&mut self, item: &I, priority: P) -> Option<P> {
        self.pq.change_priority(item, -priority)
    }

    fn pop(&mut self) -> Option<(I, P)> {
        let (item, priority) = self.pq.pop()?;

        Some((item, -priority))
    }
}
fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut blocks: Vec<Vec<i64>> = vec![];

    let mut distances: Vec<Vec<HashMap<LastMove, Infinitable<i64>>>> = vec![];
    let mut predecessors: Vec<Vec<HashMap<LastMove, (usize, usize)>>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        blocks.push(
            line.chars()
                .map(|ch| ch.to_string().parse::<i64>().unwrap())
                .collect(),
        );
        distances.push(vec![
            HashMap::from([
                (LastMove::Horizontal, Infinity),
                (LastMove::Vertical, Infinity)
            ]);
            line.chars().count()
        ]);
        predecessors.push(vec![HashMap::new(); line.chars().count()]);
    }

    distances[0][0] = HashMap::from([
        (LastMove::Horizontal, Finite(0)),
        (LastMove::Vertical, Finite(0)),
    ]);
    let mut mpq = MinPriorityQueue::new();
    mpq.push(((0usize, 0usize), LastMove::Horizontal), Finite(0i64));
    mpq.push(((0usize, 0usize), LastMove::Vertical), Finite(0i64));

    let mut discovered: HashSet<((usize, usize), LastMove)> = HashSet::new();

    while let Some(((vertex, last_move), _)) = mpq.pop() {
        if predecessors[vertex.0][vertex.1].is_empty() {
            // starting node case
            for i in 1..=3 {
                predecessors[vertex.0][vertex.1 + i].insert(LastMove::Horizontal, vertex);
                let new_distance: Infinitable<i64> = distances[vertex.0][vertex.1 + i - 1]
                    .get(&LastMove::Horizontal)
                    .map_or(Finite(0), |v| *v)
                    + Finite(blocks[vertex.0][vertex.1 + i]);
                distances[vertex.0][vertex.1 + i].insert(LastMove::Horizontal, new_distance);
                mpq.push(
                    ((vertex.0, vertex.1 + i), LastMove::Horizontal),
                    new_distance,
                );

                predecessors[vertex.0 + i][vertex.1].insert(LastMove::Vertical, vertex);
                let new_distance: Infinitable<i64> = distances[vertex.0 + i - 1][vertex.1]
                    .get(&LastMove::Vertical)
                    .map_or(Finite(0), |v| *v)
                    + Finite(blocks[vertex.0 + i][vertex.1]);
                distances[vertex.0 + i][vertex.1].insert(LastMove::Vertical, new_distance);
                mpq.push(
                    ((vertex.0 + i, vertex.1), LastMove::Horizontal),
                    new_distance,
                );
            }
        } else {
            if predecessors[vertex.0][vertex.1]
                .get(&LastMove::Horizontal)
                .is_some()
            {
                // check top and down
                for i in 1..=3 {
                    if vertex.0 >= i
                        && !discovered.contains(&((vertex.0 - i, vertex.1), LastMove::Vertical))
                    {
                        let mut new_distance: Infinitable<i64> = *distances[vertex.0][vertex.1]
                            .get(&LastMove::Horizontal)
                            .unwrap();
                        for j in 1..=i {
                            new_distance = new_distance + Finite(blocks[vertex.0 - j][vertex.1]);
                        }
                        if new_distance
                            < *distances[vertex.0 - i][vertex.1]
                                .get(&LastMove::Vertical)
                                .unwrap()
                        {
                            distances[vertex.0 - i][vertex.1]
                                .insert(LastMove::Vertical, new_distance);
                            predecessors[vertex.0 - i][vertex.1].insert(LastMove::Vertical, vertex);
                            if mpq
                                .change_priority(
                                    &((vertex.0 - i, vertex.1), LastMove::Vertical),
                                    new_distance,
                                )
                                .is_none()
                            {
                                mpq.push(
                                    ((vertex.0 - i, vertex.1), LastMove::Vertical),
                                    new_distance,
                                );
                            }
                        }
                    }
                    if vertex.0 + i < blocks.len()
                        && !discovered.contains(&((vertex.0 + i, vertex.1), LastMove::Vertical))
                    {
                        let mut new_distance: Infinitable<i64> = *distances[vertex.0][vertex.1]
                            .get(&LastMove::Horizontal)
                            .unwrap();
                        for j in 1..=i {
                            new_distance = new_distance + Finite(blocks[vertex.0 + j][vertex.1]);
                        }
                        if new_distance
                            < *distances[vertex.0 + i][vertex.1]
                                .get(&LastMove::Vertical)
                                .unwrap()
                        {
                            distances[vertex.0 + i][vertex.1]
                                .insert(LastMove::Vertical, new_distance);
                            predecessors[vertex.0 + i][vertex.1].insert(LastMove::Vertical, vertex);
                            if mpq
                                .change_priority(
                                    &((vertex.0 + i, vertex.1), LastMove::Vertical),
                                    new_distance,
                                )
                                .is_none()
                            {
                                mpq.push(
                                    ((vertex.0 + i, vertex.1), LastMove::Vertical),
                                    new_distance,
                                );
                            }
                        }
                    }
                }
            }

            if predecessors[vertex.0][vertex.1]
                .get(&LastMove::Vertical)
                .is_some()
            {
                // check left and right
                for i in 1..=3 {
                    if vertex.1 >= i
                        && !discovered.contains(&((vertex.0, vertex.1 - i), LastMove::Horizontal))
                    {
                        let mut new_distance = *distances[vertex.0][vertex.1]
                            .get(&LastMove::Vertical)
                            .unwrap();
                        for j in 1..=i {
                            new_distance = new_distance + Finite(blocks[vertex.0][vertex.1 - j]);
                        }
                        if new_distance
                            < *distances[vertex.0][vertex.1 - i]
                                .get(&LastMove::Horizontal)
                                .unwrap()
                        {
                            distances[vertex.0][vertex.1 - i]
                                .insert(LastMove::Horizontal, new_distance);
                            predecessors[vertex.0][vertex.1 - i]
                                .insert(LastMove::Horizontal, vertex);
                            if mpq
                                .change_priority(
                                    &((vertex.0, vertex.1 - i), LastMove::Horizontal),
                                    new_distance,
                                )
                                .is_none()
                            {
                                mpq.push(
                                    ((vertex.0, vertex.1 - i), LastMove::Horizontal),
                                    new_distance,
                                );
                            }
                        }
                    }
                    if vertex.1 + i < blocks[vertex.0].len()
                        && !discovered.contains(&((vertex.0, vertex.1 + i), LastMove::Horizontal))
                    {
                        let mut new_distance = *distances[vertex.0][vertex.1]
                            .get(&LastMove::Vertical)
                            .unwrap();
                        for j in 1..=i {
                            new_distance = new_distance + Finite(blocks[vertex.0][vertex.1 + j]);
                        }
                        if new_distance
                            < *distances[vertex.0][vertex.1 + i]
                                .get(&LastMove::Horizontal)
                                .unwrap()
                        {
                            distances[vertex.0][vertex.1 + i]
                                .insert(LastMove::Horizontal, new_distance);
                            predecessors[vertex.0][vertex.1 + i]
                                .insert(LastMove::Horizontal, vertex);
                            if mpq
                                .change_priority(
                                    &((vertex.0, vertex.1 + i), LastMove::Horizontal),
                                    new_distance,
                                )
                                .is_none()
                            {
                                mpq.push(
                                    ((vertex.0, vertex.1 + i), LastMove::Horizontal),
                                    new_distance,
                                );
                            }
                        }
                    }
                }
            }
        }
        discovered.insert((vertex, last_move));
    }

    let result: Infinitable<i64> = *distances
        .last()
        .unwrap()
        .last()
        .unwrap()
        .values()
        .min()
        .unwrap();

    println!("Final answer: {:?}", result);

    Ok(())
}
