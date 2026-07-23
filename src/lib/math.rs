use core::num;

// LeetCode #1291
pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut ans = vec![];
    for i in 1..=9 {
        let mut num = i;
        for j in i + 1..=9 {
            num = num * 10 + j;
            if num >= low && num <= high {
                ans.push(num);
            }
        }
    }

    ans.sort_unstable();
    ans
}
// LeetCode#
pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
    n
}
// LeetCode #1979
pub fn find_gcd(nums: Vec<i32>) -> i32 {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
    let (mut m, mut n) = (i32::MIN, i32::MAX);
    for i in nums {
        if i > m {
            m = i;
        }
        if i < n {
            n = i;
        }
    }
    gcd(m, n)
}

pub fn sum_and_multiply(n: i32) -> i64 {
    let mut x = 0;
    let mut exp = 1;
    let mut temp = n;
    let mut sum: i64 = 0;
    while temp > 0 {
        let digit = (temp % 10) as i64;
        sum += digit;
        if digit != 0 {
            x += digit * exp;
            exp *= 10;
        }
        temp /= 10;
    }
    (x * sum) as _
}

//LeetCode #3513
pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    // let n = nums.len();
    // use std::collections::HashSet;
    // let mut stt = HashSet::new();
    // for i in 0..n {
    //     for j in i..n {
    //         for k in j..n {
    //             let val = nums[i] ^ nums[j] ^ nums[k];
    //             if !stt.contains(&val) {
    //                 stt.insert(val);
    //             }
    //         }
    //     }
    // }
    // stt.len() as _
    //
    //Optimal Bit-Width Solution
    let n = nums.len();
    if n <= 2 {
        return n as _;
    } else {
        return 2i32.pow(n.ilog2() + 1);
    }
}
