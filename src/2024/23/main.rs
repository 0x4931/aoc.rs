use std::collections::{BTreeSet, HashSet};

fn create_node_list(connections: &Vec<(String, String)>) -> Vec<String> {
    let mut node_list = BTreeSet::new();

    for connection in connections.iter() {
        node_list.insert(connection.0.clone());
        node_list.insert(connection.1.clone());
    }

    Vec::from_iter(node_list)
}

fn create_adjacency_matrix(
    connections: &Vec<(String, String)>,
    node_list: &Vec<String>,
) -> Vec<Vec<bool>> {
    let nodes = node_list.len();
    let mut adjacency_matrix = vec![vec![false; nodes]; nodes];

    for connection in connections.iter() {
        let node0 = node_list.binary_search(&connection.0).unwrap();
        let node1 = node_list.binary_search(&connection.1).unwrap();
        adjacency_matrix[node0][node1] = true;
        adjacency_matrix[node1][node0] = true;
    }

    adjacency_matrix
}

fn create_adjacency_list(
    connections: &Vec<(String, String)>,
    node_list: &Vec<String>,
) -> Vec<HashSet<usize>> {
    let nodes = node_list.len();
    let mut adjacency_list = vec![HashSet::new(); nodes];

    for connection in connections.iter() {
        let node0 = node_list.binary_search(&connection.0).unwrap();
        let node1 = node_list.binary_search(&connection.1).unwrap();
        adjacency_list[node0].insert(node1);
        adjacency_list[node1].insert(node0);
    }

    adjacency_list
}

fn part_one(connections: &Vec<(String, String)>) {
    let node_list = create_node_list(connections);
    let adjacency_matrix = create_adjacency_matrix(connections, &node_list);
    let nodes = node_list.len();

    let mut count = 0;

    for i in 0..nodes {
        for j in i + 1..nodes {
            for k in j + 1..nodes {
                if adjacency_matrix[i][j] && adjacency_matrix[j][k] && adjacency_matrix[k][i] {
                    if node_list[i].starts_with('t')
                        || node_list[j].starts_with('t')
                        || node_list[k].starts_with('t')
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);
}

fn compute_maximum_clique_rec(
    mut potential_nodes: HashSet<usize>,
    current_clique: HashSet<usize>,
    mut excluded_nodes: HashSet<usize>,
    node_list: &Vec<String>,
    adjacency_list: &Vec<HashSet<usize>>,
    maximal_cliques: &mut Vec<HashSet<usize>>,
) {
    if potential_nodes.is_empty() && excluded_nodes.is_empty() {
        return maximal_cliques.push(current_clique);
    }

    while !potential_nodes.is_empty() {
        let node = *potential_nodes.iter().next().unwrap();

        let mut next_clique = current_clique.clone();
        next_clique.insert(node);

        let mut next_potential_nodes = HashSet::new();
        for other in potential_nodes.iter().cloned() {
            if adjacency_list[other].contains(&node) {
                next_potential_nodes.insert(other);
            }
        }

        let mut next_excluded_nodes = HashSet::new();
        for other in excluded_nodes.iter().cloned() {
            if adjacency_list[other].contains(&node) {
                next_excluded_nodes.insert(other);
            }
        }

        compute_maximum_clique_rec(
            next_potential_nodes,
            next_clique,
            next_excluded_nodes,
            node_list,
            adjacency_list,
            maximal_cliques,
        );

        potential_nodes.remove(&node);
        excluded_nodes.insert(node);
    }
}

/// Computes the maximum clique using the Bron-Kerbosch Algorithm
///
/// - `current_clique` is the clique being built
/// - `potential_nodes` contain all nodes that can extend the clique
/// - `excluded_nodes` contain all nodes that can extend the clique, but have already been processed
///
/// 1. For each `node` in `potential_nodes`, we will try adding it into the `current_clique`.
/// 2. Then, we will create `next_potential_nodes` and `next_excluded_nodes` by filtering
///    `potential_nodes` and `excluded_nodes` in order to keep their properties for the next
///    recursion.
/// 3. If `potential_nodes` is empty, there are no more nodes that can extend the clique.
/// 4. If `excluded_nodes` is also empty, there are no more nodes that can extend the clique, that
///    have also already been processed (the cliques resulting from extending `current_clique` using
///    this `node` are already in the output vector `maximal_cliques`).
/// 5. Therefore, when both are empty, we know we have found a new, unprocessed, **maximal clique**.
///
/// Note that a **maximal clique** is a clique that can no longer be extended. This is different from
/// the **maximum clique**, which is the clique with the largest amount of vertices.
fn compute_maximum_clique(
    node_list: &Vec<String>,
    adjacency_list: &Vec<HashSet<usize>>,
) -> HashSet<usize> {
    let nodes = node_list.len();
    let mut potential_nodes = HashSet::new();
    let current_clique = HashSet::new();
    let excluded_nodes = HashSet::new();

    for node in 0..nodes {
        potential_nodes.insert(node);
    }

    let mut maximal_cliques = Vec::new();

    compute_maximum_clique_rec(
        potential_nodes,
        current_clique,
        excluded_nodes,
        node_list,
        adjacency_list,
        &mut maximal_cliques,
    );

    maximal_cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
}

fn part_two(connections: &Vec<(String, String)>) {
    let node_list = create_node_list(connections);
    let adjacency_list = create_adjacency_list(connections, &node_list);

    let maximum_clique = compute_maximum_clique(&node_list, &adjacency_list);
    let mut maximum_clique: Vec<_> = maximum_clique
        .into_iter()
        .map(|node| node_list[node].to_owned())
        .collect();
    maximum_clique.sort();
    let maximum_clique = maximum_clique.join(",");

    println!("{}", maximum_clique);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 23)?;

    let connections: Vec<(String, String)> = input
        .lines()
        .map(|line| {
            let (comp1, comp2) = line.split_once('-').unwrap();
            let comp1 = comp1.to_owned();
            let comp2 = comp2.to_owned();
            (comp1, comp2)
        })
        .collect();

    part_one(&connections);
    part_two(&connections);

    Ok(())
}
