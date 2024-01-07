use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn find_vertexes(edges: &Vec<HashSet<String>>, start: String) -> i64 {
    let mut vertexes: HashSet<String> = HashSet::new();

    dfs(edges, &mut vertexes, start);

    vertexes.len() as i64
}

fn dfs(edges: &Vec<HashSet<String>>, vertexes: &mut HashSet<String>, start: String) {
    vertexes.insert(start.clone());

    for edge in edges {
        if edge.contains(&start) {
            let mut new_vertex = edge.clone();
            new_vertex.remove(&start);
            for v in new_vertex {
                if !vertexes.contains(&v) {
                    dfs(edges, vertexes, v);
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut edges: Vec<HashSet<String>> = vec![];

    for line in reader.lines().map_while(Result::ok) {
        let splitted: Vec<&str> = line.split(": ").collect();
        for v2 in splitted[1].split(' ') {
            edges.push(HashSet::from_iter(vec![
                splitted[0].to_string(),
                v2.to_string(),
            ]));
        }
    }

    // values found by manual analysis of the graph
    let h1 = HashSet::from_iter(vec!["scf".to_string(), "lkf".to_string()]);
    let h2 = HashSet::from_iter(vec!["pgl".to_string(), "mtl".to_string()]);
    let h3 = HashSet::from_iter(vec!["zkv".to_string(), "zxb".to_string()]);
    edges.retain(|hs| hs != &h1 && hs != &h2 && hs != &h3);

    let v1 = find_vertexes(&edges, "gbl".to_string());
    let v2 = find_vertexes(&edges, "grh".to_string());

    let result = v1 * v2;

    println!("Final answer: {:?}", result);

    Ok(())
}
