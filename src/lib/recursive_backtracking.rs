pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut sol: Vec<i32> = vec![];
    let n = nums.len();

    fn dfs(nums: &Vec<i32>, n: usize, id: usize, res: &mut Vec<Vec<i32>>, sol: &mut Vec<i32>) {
        if id == n {
            res.push(sol.clone());
            return;
        }
        //Left{Dont Pick Number[i]}
        dfs(nums, n, id + 1, res, sol);

        //Right{Pick Number[i]}
        sol.push(nums[id]);
        dfs(nums, n, id + 1, res, sol);
        sol.pop();
    }
    dfs(&nums, n, 0, &mut res, &mut sol);
    res
}
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut sol: Vec<i32> = vec![];
    let n = nums.len();

    fn dfs(nums: &Vec<i32>, n: usize, res: &mut Vec<Vec<i32>>, sol: &mut Vec<i32>) {
        if sol.len() == n {
            res.push(sol.clone());
            return;
        }
        for &i in nums {
            if !sol.contains(&i) {
                sol.push(i);
                dfs(nums, n, res, sol);
                sol.pop();
            }
        }
    }
    dfs(&nums, n, &mut res, &mut sol);
    res
}
use std::{
    collections::HashSet,
    ops::{BitOrAssign, Deref},
    str::FromStr,
    string, usize,
};
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    // Not Effiecient -- ALmost Brute Force --- Time-o(n!) // Space-o(n)
    // let mut res: Vec<(HashSet<i32>, Vec<i32>)> = vec![];
    // let mut sol: (HashSet<i32>, Vec<i32>) = (HashSet::new(), vec![]);
    //
    // fn dfs(
    //     n: i32,
    //     k: usize,
    //     res: &mut Vec<(HashSet<i32>, Vec<i32>)>,
    //     sol: &mut (HashSet<i32>, Vec<i32>),
    // ) {
    //     if sol.1.len() == k && !res.iter().any(|(x, _)| sol.0 == *x) {
    //         res.push(sol.clone());
    //         return;
    //     }
    //     for i in 1..=n {
    //         if !sol.0.contains(&i) {
    //             sol.1.push(i);
    //             sol.0.insert(i);
    //             dfs(n, k, res, sol);
    //             sol.1.pop();
    //             sol.0.remove(&i);
    //         }
    //     }
    // }
    // dfs(n, k as usize, &mut res, &mut sol);
    // res.into_iter().map(|(_, i)| i).collect()

    //Still DFS solution -- Time-o(n!) // Space-o(n) -- Without Set
    let mut res = vec![];
    let mut sol: Vec<i32> = vec![];

    fn dfs(n: i32, k: usize, res: &mut Vec<Vec<i32>>, sol: &mut Vec<i32>) {
        if sol.len() == k {
            res.push(sol.clone());
            return;
        }
        //Left Path
        let left = n;
        let still_need = k - sol.len();
        if left > still_need as i32 {
            dfs(n - 1, k, res, sol);
        }

        //Right Path
        sol.push(n);
        dfs(n - 1, k, res, sol);
        sol.pop();
    }
    dfs(n, k as usize, &mut res, &mut sol);
    res
}
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut sol: Vec<i32> = vec![];
    let nums = candidates;
    let n = nums.len();

    fn dfs(
        nums: &Vec<i32>,
        target: i32,
        i: i32,
        n: i32,
        cr_sum: i32,
        res: &mut Vec<Vec<i32>>,
        sol: &mut Vec<i32>,
    ) {
        if cr_sum == target {
            res.push(sol.clone());
            return;
        }
        if cr_sum > target || i == n {
            return;
        }
        dfs(nums, target, i + 1, n, cr_sum, res, sol);
        sol.push(nums[i as usize]);
        dfs(nums, target, i, n, cr_sum + nums[i as usize], res, sol);
        sol.pop();
    }
    dfs(&nums, target, 0, n as i32, 0, &mut res, &mut sol);
    res
}
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let mut res = vec![];
    let phone_map = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

    fn dfs(comb: String, next_digits: &str, phone_map: &Vec<&str>, output: &mut Vec<String>) {
        if next_digits.is_empty() {
            output.push(comb);
        } else {
            let letters = phone_map[next_digits.chars().nth(0).unwrap() as usize - '2' as usize];
            for i in letters.chars() {
                let new_comb = comb.clone() + &i.to_string();
                dfs(new_comb, &next_digits[1..], phone_map, output);
            }
        }
    }

    dfs(String::new(), &digits, &phone_map, &mut res);
    res
}
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    let mut sol = String::new();
    let choice = vec!['(', ')'];

    fn dfs(n: i32, sol: &mut String, res: &mut Vec<String>, open: i32, close: i32) {
        if sol.len() as i32 == 2 * n {
            res.push(sol.clone());
            return;
        }
        if open < n {
            sol.push('(');
            dfs(n, sol, res, open + 1, close);
            sol.pop();
        }
        if open > close {
            sol.push(')');
            dfs(n, sol, res, open, close + 1);
            sol.pop();
        }
    }
    dfs(n, &mut sol, &mut res, 0, 0);
    res
}
pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    let n = board[0].len();
    let w = word.len();

    if m == 1 && n == 1 {
        return board[0][0].to_string() == word;
    }
    fn dfs(
        pos: (usize, usize),
        id: usize,
        m: usize,
        n: usize,
        len_w: usize,
        board: &mut Vec<Vec<char>>,
        word: &String,
    ) -> bool {
        let (i, j) = pos;

        if id == len_w {
            return true;
        }
        if board[i][j] != word.chars().nth(id).unwrap() {
            return false;
        }

        let char = board[i][j];
        board[i][j] = '#';

        for (i_off, j_off) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (r, c) = ((i as i32 + i_off) as usize, (j as i32 + j_off) as usize);
            if (0..m).contains(&r) && (0..n).contains(&c) {
                if dfs((r, c), id + 1, m, n, len_w, board, word) {
                    return true;
                }
            }
        }
        board[i][j] = char;
        return false;
    }

    for i in 0..m {
        for j in 0..n {
            if dfs((i, j), 0, m, n, w, &mut board, &word) {
                return true;
            }
        }
    }
    return false;
}
