use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};


pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from_iter(stones.into_iter());
    while heap.len() > 1 {
        let (a, b) = (heap.pop().unwrap(), heap.pop().unwrap());
        if a != b {
            heap.push(a - b);
        }
    }
    heap.pop().unwrap_or_default()
}
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    // let mut heap = BinaryHeap::from_iter(nums.into_iter());
    // for _ in 1..k {
    //     heap.pop();
    // }
    // heap.pop().unwrap()
    let mut heap = BinaryHeap::new();
    for i in nums {
        if heap.len() >= k as usize {
            heap.push(Reverse(i));
            heap.pop();
        } else {
            heap.push(Reverse(i));
        }
    }
    heap.pop().unwrap().0
}
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //BinaryHeap Solution -- o(nlogk)
    // let mut hash = HashMap::new();
    // for i in nums {
    //     hash.entry(i).and_modify(|x| *x += 1).or_insert(1);
    // }
    // let mut heap = BinaryHeap::new();
    // for (i, n) in hash {
    //     if heap.len() >= k as usize {
    //         heap.push((Reverse(n), i));
    //         heap.pop();
    //     } else {
    //         heap.push((Reverse(n), i));
    //     }
    // }
    // let mut res = vec![];
    // while !heap.is_empty() {
    //     res.push(heap.pop().unwrap().0 .0);
    // }
    // res

    //O(n) Solution
    let mut counter = vec![vec![]; nums.len()];
    let mut hash = HashMap::new();
    let mut res = vec![];
    for i in nums {
        hash.entry(i).and_modify(|x| *x += 1).or_insert(1);
    }
    for (num, c) in hash {
        counter[c - 1].push(num);
    }
    for i in counter.iter_mut().rev() {
        if res.len() >= k as usize {
            break;
        }
        while let Some(x) = i.pop() {
            res.push(x);
        }
    }
    res
}

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut heap = BinaryHeap::new();
    for i in points {
        let d = i[0] * i[0] + i[1] * i[1];
        heap.push((d, i));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.into_iter().map(|(_, p)| p).collect()
}

pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut dist = vec![vec![i32::MAX; n]; n];
    let mut q = VecDeque::new();

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                dist[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = q.pop_front() {
        for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < n as i32 && nj >= 0 && nj < n as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if dist[ni][nj] > dist[i][j] + 1 {
                    dist[ni][nj] = dist[i][j] + 1;
                    q.push_back((ni, nj));
                }
            }
        }
    }

    let mut pq = BinaryHeap::new();
    let mut seen = vec![vec![false; n]; n];
    pq.push((dist[0][0], 0, 0));
    seen[0][0] = true;

    while let Some((d, i, j)) = pq.pop() {
        if i == n - 1 && j == n - 1 {
            return d;
        }
        for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < n as i32 && nj >= 0 && nj < n as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if !seen[ni][nj] {
                    seen[ni][nj] = true;
                    pq.push((d.min(dist[ni][nj]), ni, nj));
                }
            }
        }
    }

    0
}
