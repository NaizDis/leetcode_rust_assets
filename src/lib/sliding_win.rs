use std::{
    collections::{HashMap, HashSet},
    num,
};

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let ku = k as usize;
    let mut s = nums[..ku].iter().sum::<i32>();
    let mut max = s;

    for (&next, &pre) in nums[ku..].iter().zip(nums.iter()) {
        s += next - pre;
        max = s.max(max);
    }
    max as f64 / k as f64
}
pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    // O(n^2)
    // let mut wind_len = 0;
    // let mut i = 0;
    // while i < nums.len() {
    //     let mut cr_len = 0;
    //     let mut k_ct = k;
    //     for &e in nums[i..].iter() {
    //         if e == 0 && k_ct != 0 {
    //             cr_len += 1;
    //             k_ct -= 1;
    //         } else if e == 1 {
    //             cr_len += 1;
    //         } else {
    //             break;
    //         }
    //     }
    //     wind_len = wind_len.max(cr_len);
    //     i += 1;
    // }
    // wind_len
    let mut max_len = 0;
    let mut l = 0;
    let mut k_cr = 0;
    for i in 0..nums.len() {
        if nums[i] == 0 {
            k_cr += 1;
        }
        while k_cr > k {
            if nums[l] == 0 {
                k_cr -= 1;
            }
            l += 1;
        }
        let w = i - l + 1;
        max_len = max_len.max(w);
    }
    max_len as i32
}
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut l = 0;
    let mut max_len = 0;
    let mut stt: HashSet<char> = HashSet::new();

    for (i, c) in s.chars().enumerate() {
        while stt.contains(&c) {
            stt.remove(&(s.as_bytes()[l] as char));
            l += 1;
        }

        let w = i - l + 1;
        max_len = max_len.max(w);
        stt.insert(c);
    }
    max_len as i32
}
pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut l = 0;
    let mut max = 0;
    let mut ct = vec![0; 26];

    for (r, c) in s.chars().enumerate() {
        ct[c as usize - 65] += 1;
        while (r - l + 1) - ct.iter().max().unwrap() > k as usize {
            ct[(s.as_bytes()[l] - 65) as usize] -= 1;
            l += 1;
        }
        max = max.max(r - l + 1);
    }
    max as i32
}
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut min_len = i32::MAX;
    let mut sum = 0;
    for r in 0..nums.len() {
        sum += nums[r];
        while sum >= target {
            min_len = min_len.min((r - l + 1) as i32);
            sum -= nums[l];
            l += 1;
        }
    }

    if min_len < i32::MAX {
        min_len
    } else {
        0
    }
}
pub fn check_inclusion(s1: String, s2: String) -> bool {
    let (n1, n2) = (s1.len(), s2.len());
    if n1 > n2 {
        return false;
    }
    let mut s1_ct: Vec<u8> = vec![0; 26];
    let mut s2_ct: Vec<u8> = vec![0; 26];

    for (i, j) in s1.bytes().zip(s2.bytes()) {
        s1_ct[(i - 97) as usize] += 1;
        s2_ct[(j - 97) as usize] += 1;
    }
    if s1_ct == s2_ct {
        return true;
    }
    for (l, r) in s2.bytes().zip(s2.bytes().skip(n1)) {
        s2_ct[(r - 97) as usize] += 1;
        s2_ct[(l - 97) as usize] -= 1;
        if s1_ct == s2_ct {
            return true;
        }
    }
    false
}
