mod solution {
    use std::collections::{HashMap, HashSet};

    const MIN: i32 = 2;
    const MAX: i32 = 99;

    fn get_all_pairs() -> HashSet<(i32, i32)> {
        let mut pairs = HashSet::new();
        for i in MIN..=MAX {
            for j in i..=MAX {
                pairs.insert((i, j));
            }
        }
        pairs
    }

    fn get_sum_pairs(
        n: i32,
        sum_pairs_cache: &mut HashMap<i32, Vec<(i32, i32)>>,
    ) -> Vec<(i32, i32)> {
        if let Some(v) = sum_pairs_cache.get(&n) {
            return v.clone();
        }
        let half_n = n / 2;
        let v: Vec<(i32, i32)> = (MIN..=half_n)
            .filter(|i| (n - i) <= MAX)
            .map(|i| (i, n - i))
            .collect();
        sum_pairs_cache.insert(n, v.clone());
        v
    }

    fn get_product_pairs(
        n: i32,
        product_pairs_cache: &mut HashMap<i32, Vec<(i32, i32)>>,
    ) -> Vec<(i32, i32)> {
        if let Some(v) = product_pairs_cache.get(&n) {
            return v.clone();
        }
        let sqrt_n = (n as f64).sqrt() as i32;
        let v: Vec<(i32, i32)> = (MIN..=sqrt_n)
            .filter(|i| n % i == 0)
            .map(|i| (i, n / i))
            .collect();
        product_pairs_cache.insert(n, v.clone());
        v
    }

    fn can_be_product_number(n: i32, product_number_cache: &mut HashMap<i32, bool>) -> bool {
        if let Some(&b) = product_number_cache.get(&n) {
            return b;
        }
        let sqrt_n = (n as f64).sqrt() as i32;
        let b = (MIN..=sqrt_n).filter(|i| n % i == 0).take(2).count() == 2;
        product_number_cache.insert(n, b);
        b
    }

    fn satisfy_s1(
        (i, j): (i32, i32),
        sum_pairs_cache: &mut HashMap<i32, Vec<(i32, i32)>>,
        product_number_cache: &mut HashMap<i32, bool>,
    ) -> bool {
        let sum_pairs = get_sum_pairs(i + j, sum_pairs_cache);
        sum_pairs.len() >= 2
            && sum_pairs
                .iter()
                .all(|(x, y)| can_be_product_number(x * y, product_number_cache))
    }

    fn satisfy_s2(
        (i, j): (i32, i32),
        s1: &HashSet<(i32, i32)>,
        product_pairs_cache: &mut HashMap<i32, Vec<(i32, i32)>>,
    ) -> bool {
        let product_pairs = get_product_pairs(i * j, product_pairs_cache);
        product_pairs
            .iter()
            .filter(|&&(x, y)| s1.contains(&(x, y)))
            .take(2)
            .count()
            == 1
    }

    fn satisfy_s3(
        (i, j): (i32, i32),
        s2: &HashSet<(i32, i32)>,
        sum_pairs_cache: &mut HashMap<i32, Vec<(i32, i32)>>,
    ) -> bool {
        let sum_pairs = get_sum_pairs(i + j, sum_pairs_cache);
        sum_pairs
            .iter()
            .filter(|&&(x, y)| s2.contains(&(x, y)))
            .take(2)
            .count()
            == 1
    }

    fn sieve<F>(set: &HashSet<(i32, i32)>, mut predicate: F) -> HashSet<(i32, i32)>
    where
        F: FnMut((i32, i32)) -> bool,
    {
        set.iter()
            .filter(|&&(i, j)| predicate((i, j)))
            .cloned()
            .collect()
    }

    pub fn solve() -> HashSet<(i32, i32)> {
        let mut sum_pairs_cache = HashMap::new();
        let mut product_pairs_cache = HashMap::new();
        let mut product_number_cache = HashMap::new();

        let all_pairs = get_all_pairs();
        let s1 = sieve(&all_pairs, |pair| {
            satisfy_s1(pair, &mut sum_pairs_cache, &mut product_number_cache)
        });
        let s2 = sieve(&s1, |pair| satisfy_s2(pair, &s1, &mut product_pairs_cache));
        let s3 = sieve(&s2, |pair| satisfy_s3(pair, &s2, &mut sum_pairs_cache));
        s3
    }
}

fn main() {
    let ans = solution::solve();
    print!("{:?}", ans);
}
