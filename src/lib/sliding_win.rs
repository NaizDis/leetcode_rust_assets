use std::num;

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
