use std::collections::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::<String, u64>::new();

    s.into_iter()
     .for_each(|name| *map.entry(name).or_default() += 1);

    let ans = map.into_iter()
                 .fold((String::new(), 0), |(acc_name, acc_count), (name, count)| if acc_count < count {(name, count)} else {(acc_name, acc_count)});

    println!("{}", ans.0);
}
