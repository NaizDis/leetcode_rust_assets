use core::num;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ops::RangeBounds,
};

// LeetCode #977
pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
    // nums.sort_by_key(|x| x.abs());
    // nums.iter().map(|&e| e * e).collect()
    let mut res = vec![0; nums.len()];
    if !nums.is_empty() {
        let mut left = nums.iter().map(|x| x.pow(2)).peekable();
        let mut right = nums.iter().rev().map(|x| x.pow(2)).peekable();
        for i in (0..nums.len()).rev() {
            res[i] = if left.peek().unwrap() > right.peek().unwrap() {
                left.next().unwrap()
            } else {
                right.next().unwrap()
            }
        }
    }
    res
}
// LeetCode #344
pub fn reverse_string(s: &mut Vec<char>) {
    let (mut i, mut j) = (0, s.len() - 1);
    while j > i {
        let temp = s[i];
        s[i] = s[j];
        s[j] = temp;
        i += 1;
        j -= 1;
    }
}
// LeetCode #167
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut i, mut j) = (0, numbers.len() - 1);
    while j > i {
        let obj = numbers[i] + numbers[j];
        if obj > target {
            j -= 1;
        } else if obj < target {
            i += 1;
        } else {
            return vec![i as i32 + 1, j as i32 + 1];
        }
    }
    vec![]
}
// LeetCode #125
pub fn is_palindrome(mut s: String) -> bool {
    s = s.to_lowercase();
    let chars: Vec<char> = s.chars().filter(|e| e.is_alphanumeric()).collect();
    if chars.len() == 0 {
        return true;
    }
    let (mut i, mut j) = (0, chars.len() - 1);
    while j > i {
        if chars[i] == chars[j] {
            j -= 1;
            i += 1;
        } else {
            return false;
        }
    }
    true
}
// LeetCode #15
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = HashSet::new();
    let mut hash = HashMap::new();
    for i in 0..nums.len() {
        hash.insert(nums[i], i);
    }
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let sum = nums[i] + nums[j];
            if hash.contains_key(&-sum)
                && i != *hash.get(&-sum).unwrap()
                && j != *hash.get(&-sum).unwrap()
            {
                let mut vc = vec![nums[i], nums[j], -sum];
                vc.sort();
                res.insert(vc);
            }
        }
    }
    let mut fp: Vec<Vec<i32>> = vec![];
    for i in res {
        fp.push(i);
    }
    fp
}

// LeetCode #11
pub fn max_area(mut height: Vec<i32>) -> i32 {
    // let mut res = 0;
    // for i in 0..height.len() {
    //     for j in i + 1..height.len() {
    //         let cr_area = (j - i) as i32 * (height[i].min(height[j]));
    //         res = res.max(cr_area);
    //     }
    // }
    // res
    let mut res = 0;
    let (mut i, mut j) = (0, height.len() - 1);
    while j > i {
        let cr_area = (j - i) as i32 * (height[i].min(height[j]));
        res = res.max(cr_area);
        if height[i] > height[j] {
            j -= 1;
        } else {
            i += 1;
        }
    }
    res
}
// LeetCode #42
pub fn trap(height: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = height.len();
    let mut l_wall = 0;
    let mut r_wall = 0;
    let mut v_l = vec![0; n];
    let mut v_r = vec![0; n];

    //Init Walls Vector
    for i in 0..n {
        let j = n - i - 1;
        v_l[i] = l_wall;
        v_r[j] = r_wall;
        l_wall = l_wall.max(height[i]);
        r_wall = r_wall.max(height[j]);
    }

    //Found sum
    for i in 0..n {
        let pot = v_r[i].min(v_l[i]);
        res += 0.max(pot - height[i]);
    }
    res
}
// LeetCode #283
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let (mut l, n) = (0, nums.len());
    for r in 0..n {
        if nums[r] != 0 {
            let a = nums[r];
            nums[r] = nums[l];
            nums[l] = a;
            l += 1;
        }
    }
}
// LeetCode#
pub fn gcd_sum(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut pref_sum = vec![1; n];
    let mut mx_array = vec![i32::MIN; n];
    let mut ans = 0;

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    let mut maxTill = nums[0];
    for i in 0..n {
        if maxTill < nums[i] {
            mx_array[i] = nums[i];
            maxTill = nums[i];
        } else {
            mx_array[i] = maxTill;
        }
        pref_sum[i] = gcd(mx_array[i] as _, nums[i] as _);
    }

    pref_sum.sort_unstable();

    let (mut l, mut r) = (0, n - 1);

    while r > l {
        ans += gcd(pref_sum[l], pref_sum[r]);
        l += 1;
        r -= 1;
    }
    ans as _
}
