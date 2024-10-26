use std::collections::{HashMap, HashSet, VecDeque};

use crate::lib::graphs;

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    if source == destination {
        return true;
    }

    //Adjacency List Using Hashmap
    let mut graph = HashMap::new();
    for ed in edges.iter() {
        graph.insert(ed[0], ed[1]);
    }

    let mut set = HashSet::new();
    set.insert(source);

    fn dfs(i: i32, destination: i32, graph: &HashMap<i32, i32>, set: &mut HashSet<i32>) -> bool {
        if i == destination {
            return true;
        }
        for &nigh in graph.get(&i) {
            if !set.contains(&nigh) {
                set.insert(nigh);
                if dfs(nigh, destination, graph, set) {
                    return true;
                }
            }
        }
        false
    }
    dfs(source, destination, &graph, &mut set)
}
pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut res = 0;

    fn dfs(i: i32, j: i32, m: i32, n: i32, grid: &mut Vec<Vec<char>>) {
        if i < 0 || i >= m || j < 0 || j >= n || grid[i as usize][j as usize] != '1' {
        } else {
            grid[i as usize][j as usize] = '0';
            dfs(i, j + 1, m, n, grid);
            dfs(i + 1, j, m, n, grid);
            dfs(i, j - 1, m, n, grid);
            dfs(i - 1, j, m, n, grid);
        }
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                res += 1;
                dfs(i as i32, j as i32, m as i32, n as i32, grid.as_mut());
            }
        }
    }
    res
}
pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut res = 0;

    fn dfs(i: i32, j: i32, m: i32, n: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
        if i < 0 || i >= m || j < 0 || j >= n || grid[i as usize][j as usize] != 1 {
            0
        } else {
            grid[i as usize][j as usize] = 0;
            1 + dfs(i, j + 1, m, n, grid)
                + dfs(i + 1, j, m, n, grid)
                + dfs(i, j - 1, m, n, grid)
                + dfs(i - 1, j, m, n, grid)
        }
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                res = res.max(dfs(i as i32, j as i32, m as i32, n as i32, grid.as_mut()));
            }
        }
    }
    res
}
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for ed in prerequisites.iter() {
        graph
            .entry(ed[0])
            .and_modify(|e| e.push(ed[1]))
            .or_insert(vec![ed[1]]);
    }
    #[derive(Debug, Clone, Copy)]
    enum State {
        VISITING,
        VISITED,
        UNVISISTED,
    }
    let mut states = vec![State::UNVISISTED; num_courses as usize];

    fn dfs(node: usize, states: &mut Vec<State>, graph: &HashMap<i32, Vec<i32>>) -> bool {
        let state = states[node];
        match state {
            State::VISITED => true,
            State::VISITING => false,
            State::UNVISISTED => {
                states[node] = State::VISITING;
                if let Some(nigh) = graph.get(&(node as i32)) {
                    for &n in nigh {
                        if !dfs(n as usize, states, graph) {
                            return false;
                        }
                    }
                };
                states[node] = State::VISITED;
                true
            }
        }
    }

    for i in 0..num_courses {
        if !dfs(i as usize, states.as_mut(), &graph) {
            return false;
        }
    }
    true
}
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for ed in prerequisites.iter() {
        graph
            .entry(ed[0])
            .and_modify(|e| e.push(ed[1]))
            .or_insert(vec![ed[1]]);
    }
    #[derive(Debug, Clone, Copy)]
    enum State {
        VISITING,
        VISITED,
        UNVISISTED,
    }
    let mut states = vec![State::UNVISISTED; num_courses as usize];

    fn dfs(
        node: usize,
        states: &mut Vec<State>,
        graph: &HashMap<i32, Vec<i32>>,
        order: &mut Vec<i32>,
    ) -> bool {
        let state = states[node];
        match state {
            State::VISITED => true,
            State::VISITING => false,
            State::UNVISISTED => {
                states[node] = State::VISITING;
                if let Some(nigh) = graph.get(&(node as i32)) {
                    for &n in nigh {
                        if !dfs(n as usize, states, graph, order) {
                            return false;
                        }
                    }
                };
                states[node] = State::VISITED;
                order.push(node as i32);
                true
            }
        }
    }

    for i in 0..num_courses {
        if !dfs(i as usize, states.as_mut(), &graph, &mut res) {
            return vec![];
        }
    }
    res
}
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (heights.len(), heights[0].len());
    let mut res = vec![];

    let mut p_que = VecDeque::new();
    let mut p_seen = HashSet::new();
    let mut a_que = VecDeque::new();
    let mut a_seen = HashSet::new();

    for j in 0..n {
        p_que.push_back((0, j));
        p_seen.insert((0, j));
    }
    for i in 1..m {
        p_que.push_back((i, 0));
        p_seen.insert((i, 0));
    }
    for j in 0..n - 1 {
        a_que.push_back((m - 1, j));
        a_seen.insert((m - 1, j));
    }
    for i in 0..m {
        a_que.push_back((i, n - 1));
        a_seen.insert((i, n - 1));
    }

    fn bfs(
        que: &mut VecDeque<(usize, usize)>,
        seen: &mut HashSet<(usize, usize)>,
        grid: &Vec<Vec<i32>>,
        m: i32,
        n: i32,
    ) {
        while !que.is_empty() {
            let (i, j) = que.pop_front().unwrap();

            for (i_off, j_off) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let (r, c) = (i as i32 + i_off, j as i32 + j_off);
                if 0 <= r
                    && r < m
                    && 0 <= c
                    && c < n
                    && grid[r as usize][c as usize] >= grid[i][j]
                    && !seen.contains(&(r as usize, c as usize))
                {
                    seen.insert((r as usize, c as usize));
                    que.push_back((r as usize, c as usize));
                }
            }
        }
    }
    bfs(&mut a_que, &mut a_seen, &heights, m as i32, n as i32);
    bfs(&mut p_que, &mut p_seen, &heights, m as i32, n as i32);

    for (p, q) in a_seen.intersection(&p_seen) {
        res.push(vec![*p as i32, *q as i32]);
    }

    res
}
pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut que = VecDeque::new();
    let mut fresh_num = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 2 {
                que.push_back((i, j));
            } else if grid[i][j] == 1 {
                fresh_num += 1;
            }
        }
    }
    if fresh_num == 0 {
        return 0;
    }
    let mut min = -1;
    while !que.is_empty() {
        let q_s = que.len();
        min += 1;
        for _ in 0..q_s {
            let (i, j) = que.pop_front().unwrap();
            for (r, c) in [(i, j + 1), (i + 1, j), (i - 1, j), (i, j - 1)] {
                if 0 <= r && r < m && 0 <= c && c < n && grid[r][c] == 1 {
                    grid[r][c] = 2;
                    fresh_num -= 1;
                    que.push_back((r, c));
                }
            }
        }
    }
    if fresh_num == 0 {
        min
    } else {
        -1
    }
}
