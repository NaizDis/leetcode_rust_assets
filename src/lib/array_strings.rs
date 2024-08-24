use std::{cmp, collections::HashMap, i32::MIN};
pub fn merge_alternately(word1: String, word2: String) -> String {
    word1
        .chars()
        .zip(word2.chars())
        .flat_map(|(c1, c2)| [c1, c2])
        .chain(word1.chars().skip(word2.len()))
        .chain(word2.chars().skip(word1.len()))
        .collect()
}

pub fn roman_to_int(s: String) -> i32 {
    let mut res = 0;
    let chars: Vec<char> = s.chars().collect();
    let hash = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
        ('I', 1),
    ]);

    for i in 0..chars.len() - 1 {
        if hash[&chars[i]] < hash[&chars[i + 1]] {
            res -= hash[&chars[i]];
        } else {
            res += hash[&chars[i]];
        }
    }
    res += hash[&chars[chars.len() - 1]];
    res
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let mut chrs: Vec<char> = s.chars().collect();
    for i in t.chars() {
        if chrs.is_empty() {
            break;
        } else if chrs[0] == i {
            chrs.remove(0);
        }
    }
    chrs.is_empty()
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut bday = prices[0];
    let mut profit: i32 = 0;
    for &i in prices.iter().skip(1) {
        if i < bday {
            bday = i;
        } else {
            let pot = i - bday;
            if pot > profit {
                profit = pot;
            }
        }
    }
    profit
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut min_len: usize = usize::MAX;
    for i in &strs {
        if i.len() < min_len {
            min_len = i.len();
        }
    }
    let mut p = 0;
    'pp: while p < min_len {
        for j in &strs {
            if j.chars().nth(p) != strs[0].chars().nth(p) {
                break 'pp;
            }
        }
        p += 1;
    }
    strs[0][..p].to_string()
}
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut i = 0;
    while i < nums.len() {
        let start = nums[i];
        while i < nums.len() - 1 && nums[i] + 1 == nums[i + 1] {
            i += 1;
        }
        if nums[i] == start {
            res.push(format!("{}", start));
        } else {
            res.push(format!("{}->{}", start, nums[i]));
        }
        i += 1;
    }
    res
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut l_mult: i32 = 1;
    let mut r_mult: i32 = 1;
    let n = nums.len();
    let mut l_arr = Vec::new();
    let mut r_arr = Vec::new();

    for i in 0..n {
        let j = n - 1 - i;
        l_arr.push(l_mult);
        r_arr.push(r_mult);
        l_mult *= nums[i];
        r_mult *= nums[j];
    }
    r_arr.reverse();

    l_arr.iter().zip(&r_arr).map(|(a, b)| a * b).collect()
}

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|q, r| q[0].cmp(&r[0]));
    let mut res: Vec<Vec<i32>> = Vec::new();

    for interval in intervals {
        let prev = match res.last_mut() {
            Some(a) => a,
            None => {
                res.push(interval);
                continue;
            }
        };

        if prev[1] >= interval[0] {
            *prev = vec![prev[0], std::cmp::max(prev[1], interval[1])];
        } else {
            res.push(interval);
        }
    }
    res
}

pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering;
    nums.into_iter()
        .max_by(|x, y| match y.abs().cmp(&(x.abs())) {
            Ordering::Equal => x.cmp(y),
            abs_cmp => abs_cmp,
        })
        .unwrap()
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    let n = matrix[0].len() * matrix.len();
    let mut i = 0;
    let mut j = 0;
    enum Dir {
        UP,
        RIGHT,
        DOWN,
        LEFT,
    }
    let mut dir = Dir::RIGHT;

    let mut r_wall = matrix[0].len();
    let mut l_wall: i32 = -1;
    let mut floor = matrix.len();
    let mut ceil = 0;

    while res.len() != n {
        match dir {
            Dir::RIGHT => {
                while j < r_wall {
                    res.push(matrix[i][j]);
                    j += 1;
                }
                i += 1;
                j -= 1;
                r_wall -= 1;
                dir = Dir::DOWN;
            }
            Dir::DOWN => {
                while i < floor {
                    res.push(matrix[i][j]);
                    i += 1;
                }
                i -= 1;
                j -= 1;
                floor -= 1;
                dir = Dir::LEFT;
            }
            Dir::UP => {
                while i > ceil {
                    res.push(matrix[i][j]);
                    i -= 1;
                }
                i += 1;
                j += 1;
                ceil += 1;
                dir = Dir::LEFT;
            }
            Dir::LEFT => {
                while j as i32 > l_wall {
                    res.push(matrix[i][j]);
                    j -= 1;
                }
                i -= 1;
                j += 1;
                l_wall += 1;
                dir = Dir::UP;
            }
        }
    }
    res
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let dim = matrix.len();
    for i in 0..dim {
        for j in i + 1..dim {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
    for i in matrix {
        i.reverse();
    }
}
pub fn larg_arith_sub(nums: Vec<i32>) -> i32 {
    let mut res = 2;
    let mut cur = 2;
    let mut i = 1;
    while i < nums.len() - 1 {
        if nums[i] - nums[i - 1] == nums[i + 1] - nums[i] {
            cur += 1;
        }
        res = cmp::max(res, cur);
        i += 1;
    }
    res
}
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max: i32 = MIN;
    let mut sum = 0;
    for i in nums {
        sum += i;
        max = max.max(sum);
        if sum < 0 {
            sum = 0;
        }
    }
    max
}
