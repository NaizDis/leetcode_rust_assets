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
pub fn unique_xor_triplets1(nums: Vec<i32>) -> i32 {
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

//LeetCode #3514
pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let max_xor = 2048;

    let mut pair_xor = vec![false; max_xor];
    let mut trip_xor = vec![false; max_xor];

    for i in 0..n {
        for j in i..n {
            pair_xor[(nums[i] ^ nums[j]) as usize] = true;
        }
    }

    for val in 0..max_xor {
        if !pair_xor[val] {
            continue;
        }
        for x in &nums {
            trip_xor[(x ^ val as i32) as usize] = true;
        }
    }

    let mut cnt = 0;
    for i in trip_xor {
        if i {
            cnt += 1
        };
    }
    cnt
}

//Leetocde 3536
pub fn max_product(n: i32) -> i32 {
    let mut digits = vec![];
    let mut temp = n;
    while temp != 0 {
        digits.push(temp % 10);
        temp /= 10;
    }
    digits.sort();
    digits[digits.len() - 1] * digits[digits.len() - 2]
}

//Leetocde #628
pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut nums_sort = nums.clone();
    nums_sort.sort();
    let n = nums.len();
    let neg_max = nums_sort[0] * nums_sort[1] * nums_sort[n - 1];
    let pos_max = nums_sort[n - 1] * nums_sort[n - 2] * nums_sort[n - 3];
    neg_max.max(pos_max)
}

//Leetocde #3041
pub fn minimum_pushes(word: String) -> i32 {
    let quo = word.len() / 8;
    let rem = word.len() % 8;
    ((4 * quo + rem) * (quo + 1)) as _
}

//Leetocde #3345
pub fn smallest_number(n: i32, t: i32) -> i32 {
    let mut smallest = n;
    loop {
        let mut temp = smallest;
        let mut prod: i64 = 1;
        while temp > 0 {
            prod *= (temp % 10) as i64;
            temp /= 10;
        }
        if prod % t as i64 == 0 {
            return smallest;
        }
        smallest += 1;
    }
}

//Leetocde #2778
pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let n = nums.len();
    for i in 0..n {
        if n % (i + 1) == 0 {
            sum += nums[i] * nums[i];
        }
    }
    sum
}
// Leetocde #3723
pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
    // if sum > 9 * num {
    //     return String::from("");
    // }
    // let amt9 = sum / 9;
    // let md = sum % 9;
    // let mut ans = "9".repeat(amt9 as usize);
    // let mut after9 = num - amt9;
    // if after9 <= 0 && md > 0 {
    //     return String::from("");
    // }
    // if md == 0 && after9 <= 0 {
    //     return ans;
    // }
    // ans.push_str(&md.to_string());
    // after9 -= 1;
    //
    // for _ in 0..after9 {
    //     ans.push_str("0");
    // }
    // ans

    //Refactor
    if sum > 9 * num {
        return String::new();
    }
    let mut ans = "9".repeat((sum / 9) as usize);
    let rem = (sum % 9) as u8;
    if rem > 0 {
        ans.push((b'0' + rem) as char);
    }
    while ans.len() < num as usize {
        ans.push('0');
    }
    ans
}
