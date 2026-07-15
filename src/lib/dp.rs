pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }
    let mut a = 1;
    let mut b = 2;
    for _ in 2..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut min_cost = vec![0, 0];
    for i in 2..n + 1 {
        let next_cost = (min_cost[i - 2] + cost[i - 2]).min(min_cost[i - 1] + cost[i - 1]);
        min_cost.push(next_cost);
    }
    min_cost[n]
}
pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    if n == 2 {
        return nums[0].max(nums[1]);
    }

    let mut res = vec![nums[0], nums[0].max(nums[1])];
    for i in 2..n {
        let crr = res[i - 1].max(nums[i] + res[i - 2]);
        res.push(crr);
    }
    *res.iter().last().unwrap()
}
pub fn unique_paths(m: i32, n: i32) -> i32 {
    // Recursive Solution -- Time Limit Exceeded -- O(2^(m+n))
    // if m == 1 || n == 1 {
    //     return 1;
    // }
    // unique_paths(m - 1, n) + unique_paths(m, n - 1)
    //
    // Dynamic Programming -- O(m*n)
    let mut dp = vec![vec![0; n as usize]; m as usize];
    for i in 0..m as usize {
        dp[i][0] = 1;
    }
    for i in 0..n as usize {
        dp[0][i] = 1;
    }
    for i in 1..m as usize {
        for j in 1..n as usize {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[(m - 1) as usize][(n - 1) as usize]
}
pub fn can_jump(nums: Vec<i32>) -> bool {
    //O(n) -- Greedy Approach
    let n = nums.len();
    // let mut max_reach = 0;
    // for i in 0..n {
    //     if i > max_reach {
    //         return false;
    //     }
    //     max_reach = max_reach.max(i + nums[i] as usize);
    // }
    // true
    //
    //End Greed Approach
    let mut goal = n - 1;

    for i in (0..n - 1).rev() {
        if i + nums[i] as usize >= goal {
            goal = i;
        }
    }

    goal == 0
}
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // Dynamic Programming -- O(n*amount)
    let mut dp = vec![amount + 1; amount as usize + 1];
    dp[0] = 0;
    for i in 1..=amount as usize {
        for coin in &coins {
            if i >= *coin as usize {
                dp[i] = dp[i].min(dp[i - *coin as usize] + 1);
            }
        }
    }
    if dp[amount as usize] > amount {
        -1
    } else {
        dp[amount as usize]
    }
}
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let n = coins.len();
    let mut dp = vec![vec![0; amount as usize + 1]; n + 1];

    //base case
    //we cannot make amount 0 for any case
    for i in 0..=n {
        dp[i][0] = 1;
    }

    for i in 1..=n {
        for j in 1..=amount {
            let j = j as usize;
            if coins[i - 1] <= j as i32 {
                dp[i][j] = dp[i][j - (coins[i - 1] as usize)] + dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    dp[n][amount as usize]
}
pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    use std::collections::HashMap;
    let mut dp = HashMap::new();
    dp.insert((0i32, 0i32), 1i64);

    for &x in &nums {
        let prev: Vec<_> = dp.drain().collect();
        for ((g1, g2), cnt) in prev {
            // skip
            *dp.entry((g1, g2)).or_default() =
                (dp.get(&(g1, g2)).copied().unwrap_or(0) + cnt) % MOD;
            // put in seq1
            let ng1 = if g1 == 0 { x } else { gcd(g1, x) };
            *dp.entry((ng1, g2)).or_default() =
                (dp.get(&(ng1, g2)).copied().unwrap_or(0) + cnt) % MOD;
            // put in seq2
            let ng2 = if g2 == 0 { x } else { gcd(g2, x) };
            *dp.entry((g1, ng2)).or_default() =
                (dp.get(&(g1, ng2)).copied().unwrap_or(0) + cnt) % MOD;
        }
    }

    let mut ans: i64 = 0;
    for ((g1, g2), cnt) in &dp {
        if g1 == g2 && *g1 != 0 {
            ans = (ans + cnt) % MOD;
        }
    }
    ans as i32
}
