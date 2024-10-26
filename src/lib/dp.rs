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
