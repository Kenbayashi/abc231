fn main() {
    proconio::input! {
        n: usize,
        x: u64,
        coins: [u64; n],
    }

    let ans = count_coin(0, n, &coins, x);

    println!("{}", ans);
}

fn count_coin(index: usize, n: usize, coins: &Vec<u64>, x: u64) -> u64 {
    if index == n - 1 {
        return x / coins[index];
    }

    let payment = x % coins[index + 1];
    let charge  = coins[index + 1] - payment;

    let under_case = count_coin(index + 1, n, coins, x - payment) + payment / coins[index];
    let over_case = count_coin(index + 1, n, coins, x + charge) + charge / coins[index];

    u64::min(under_case, over_case)
}
