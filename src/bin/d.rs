use std::collections::HashMap;

fn main() {
    proconio::input! {
        _: usize,
        m: usize,
        xs: [u64; m * 2],
    }

    let mut map = HashMap::<u64, u8>::new();

    xs.into_iter()
      .for_each(|x| *map.entry(x).or_default() += 1);

    let ans = map.into_iter()
                 .fold(true, |acc, (_, count)| acc && count <= 2);

    println!("{}", if ans {"Yes"} else {"No"});
}
