fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        a: [u64; n],
        x: [u64; q],
    }

    let get_ans = |num_x| {
        let ans = a.iter()
                   .filter(|&num_a| num_x <= num_a)
                   .count();

        println!("{}", ans);
    };

    x.iter()
     .for_each(get_ans);
}
