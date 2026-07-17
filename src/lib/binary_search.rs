use core::num;
use std::{cmp::Ordering, usize};

use super::linklist::middle_node;
// LeetCode #704
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] == target {
            return m as i32;
        } else if nums[m as usize] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    return -1 as i32;
}
// LeetCode #35
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] == target {
            return m as i32;
        } else if nums[m as usize] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    if l == 0 && r == 0 {
        0
    } else {
        l + (r - l) / 2 + 1
    }
}
// LeetCode #367
pub fn is_perfect_square(num: i32) -> bool {
    let (mut l, mut r) = (0, num);
    while r >= l {
        let m = (l + r) / 2;
        let sq = m * m;
        match sq.cmp(&num) {
            Ordering::Equal => return true,
            Ordering::Greater => r = m - 1,
            Ordering::Less => l = m + 1,
        }
    }
    false
}
// LeetCode #74
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    let mut start = 0;
    let mut end = rows * cols - 1;

    while start <= end {
        let mid = start + (end - start) / 2;
        let val = matrix[(mid / cols) as usize][(mid % cols) as usize];

        if val == target {
            return true;
        } else if val > target {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    false
}
// LeetCode #153
pub fn find_min(nums: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0 as i32, (nums.len() - 1) as i32);
    let mut m = 0;

    while r > l {
        m = (r + l) / 2;

        if nums[m as usize] > nums[r as usize] {
            l = m + 1;
        } else {
            r = m;
        }
    }
    nums[l as usize]
}
// LeetCode #33
pub fn search_rot(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0 as i32, (nums.len() - 1) as i32);
    let mut m = 0;
    let n = nums.len() as i32;

    while r > l {
        m = (r + l) / 2;

        if nums[m as usize] > nums[r as usize] {
            l = m + 1;
        } else {
            r = m;
        }
    }

    let mut start: i32;
    let mut end: i32;

    if l == 0 {
        start = 0;
        end = n - 1;
    } else if target >= nums[0] && target <= nums[(l - 1) as usize] {
        start = 0;
        end = l - 1;
    } else {
        start = l;
        end = n - 1;
    }
    let mut l = start;
    let mut r = end;

    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m as usize] == target {
            return m as i32;
        } else if nums[m as usize] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    return -1 as i32;
}
// LeetCode #875
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut max = *piles.iter().max().unwrap();
    let mut l = 1;

    while max > l {
        let m = (max + l) / 2;
        if k_test(&piles, h, m) {
            max = m;
        } else {
            l = m + 1;
        }
    }
    l
}
pub fn k_test(pp: &Vec<i32>, hr: i32, k: i32) -> bool {
    let mut hrs = 0;
    for i in pp {
        hrs += (*i as f32 / k as f32).ceil() as i32;
    }
    hrs <= hr
}
// LeetCode #69
pub fn my_sqrt(x: i32) -> i32 {
    if x <= 1 {
        return x;
    }
    let (mut l, mut h) = (1, x);
    let mut m = -1;
    let mut res = 0;
    while h >= l {
        m = (l + h) / 2;
        match (m).cmp(&(x / m)) {
            Ordering::Less => {
                l = m + 1;
                res = m;
            }
            Ordering::Greater => {
                h = m - 1;
            }
            Ordering::Equal => {
                return m;
            }
        }
    }
    res
}
// LeetCode #34
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if let Ok(_) = nums.binary_search(&target) {
        vec![
            nums.partition_point(|&i| i < target) as i32,
            nums.partition_point(|&i| i <= target) as i32,
        ]
    } else {
        vec![-1, -1]
    }
}
// LeetCode #3312
pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
    // O(n^2) brute force – kept for reference (TLE on large inputs)
    // let n = nums.len();
    // let mut gcd_arr = vec![];

    //     while b != 0 {
    //         let temp = b;
    //         b = a % b;
    //         a = temp;
    //     }
    //     a
    // }
    //
    // for i in 0..n {
    //     for j in i + 1..n {
    //         gcd_arr.push(gcd(nums[i], nums[j]));
    //     }
    // }
    // let mut ans = vec![0; queries.len()];
    //
    // gcd_arr.sort_unstable();
    //
    // for i in 0..queries.len() {
    //     ans[i] = gcd_arr[queries[i] as usize];
    // }
    //
    // ans

    let max_val = *nums.iter().max().unwrap_or(&0) as usize;

    // freq[x] = how many times x appears in nums
    let mut freq = vec![0i64; max_val + 1];
    for &x in &nums {
        freq[x as usize] += 1;
    }

    // cnt_gcd[g] = number of pairs with GCD exactly = g
    let mut cnt_gcd = vec![0i64; max_val + 1];

    // Process g from largest to smallest so that cnt_gcd[2g], cnt_gcd[3g], …
    // are already computed when we need to subtract them.
    for g in (1..=max_val).rev() {
        // Count how many numbers in nums are divisible by g
        let mut v = 0i64;
        let mut j = g;
        while j <= max_val {
            v += freq[j];
            j += g;
        }

        // C(v, 2) – pairs where both numbers are divisible by g.
        // Their GCD is a multiple of g (not necessarily exactly g).
        cnt_gcd[g] = v * (v - 1) / 2;

        // Subtract pairs whose GCD is a larger multiple of g (2g, 3g, …),
        // leaving only pairs with exact GCD = g.
        let mut j = 2 * g;
        while j <= max_val {
            cnt_gcd[g] -= cnt_gcd[j];
            j += g;
        }
    }

    // pref[g] = total number of pairs with GCD <= g
    let mut pref = vec![0i64; max_val + 1];
    for g in 1..=max_val {
        pref[g] = pref[g - 1] + cnt_gcd[g];
    }

    // For each query, find the GCD value by binary searching on prefix sums
    let mut ans = vec![0i32; queries.len()];
    for (i, &q) in queries.iter().enumerate() {
        let (mut lo, mut hi) = (1, max_val);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if pref[mid] > q {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        ans[i] = lo as i32;
    }

    ans
}
