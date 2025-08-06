mod solution {
    use std::collections::{HashMap, HashSet};
    use std::sync::Mutex;

    const MIN: i32 = 2;
    const MAX: i32 = 99;

    use once_cell::sync::Lazy;

    static SUM_PAIRS_CACHE: Lazy<Mutex<HashMap<i32, Vec<(i32, i32)>>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));
    static PRODUCT_PAIRS_CACHE: Lazy<Mutex<HashMap<i32, Vec<(i32, i32)>>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));
    static PRODUCT_NUMBER_CACHE: Lazy<Mutex<HashMap<i32, bool>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    fn get_all_pairs() -> HashSet<(i32, i32)> {
        let mut pairs = HashSet::new();
        for i in MIN..=MAX {
            for j in i..=MAX {
                pairs.insert((i, j));
            }
        }
        pairs
    }

    fn get_sum_pairs(n: i32) -> Vec<(i32, i32)> {
        let mut cache = SUM_PAIRS_CACHE.lock().unwrap();
        if let Some(v) = cache.get(&n) {
            return v.clone();
        }
        let half_n = n / 2;
        let v: Vec<(i32, i32)> = (MIN..=half_n)
            .filter(|i| (n - i) <= MAX)
            .map(|i| (i, n - i))
            .collect();
        cache.insert(n, v.clone());
        v
    }

    fn get_product_pairs(n: i32) -> Vec<(i32, i32)> {
        let mut cache = PRODUCT_PAIRS_CACHE.lock().unwrap();
        if let Some(v) = cache.get(&n) {
            return v.clone();
        }
        let sqrt_n = (n as f64).sqrt() as i32;
        let v: Vec<(i32, i32)> = (MIN..=sqrt_n)
            .filter(|i| n % i == 0)
            .map(|i| (i, n / i))
            .collect();
        cache.insert(n, v.clone());
        v
    }

    fn can_be_product_number(n: i32) -> bool {
        let mut cache = PRODUCT_NUMBER_CACHE.lock().unwrap();
        if let Some(&b) = cache.get(&n) {
            return b;
        }
        let sqrt_n = (n as f64).sqrt() as i32;
        let b = (MIN..=sqrt_n).filter(|i| n % i == 0).take(2).count() == 2;
        cache.insert(n, b);
        b
    }

    fn satisfy_s1((i, j): (i32, i32)) -> bool {
        let sum_pairs = get_sum_pairs(i + j);
        sum_pairs.len() >= 2 && sum_pairs.iter().all(|(x, y)| can_be_product_number(x * y))
    }

    fn satisfy_s2((i, j): (i32, i32), s1: &HashSet<(i32, i32)>) -> bool {
        let product_pairs = get_product_pairs(i * j);
        product_pairs
            .iter()
            .filter(|&&(x, y)| s1.contains(&(x, y)))
            .take(2)
            .count()
            == 1
    }

    fn satisfy_s3((i, j): (i32, i32), s2: &HashSet<(i32, i32)>) -> bool {
        let sum_pairs = get_sum_pairs(i + j);
        sum_pairs
            .iter()
            .filter(|&&(x, y)| s2.contains(&(x, y)))
            .take(2)
            .count()
            == 1
    }

    fn sieve<F>(set: &HashSet<(i32, i32)>, predicate: F) -> HashSet<(i32, i32)>
    where
        F: Fn((i32, i32)) -> bool,
    {
        set.iter()
            .filter(|&&(i, j)| predicate((i, j)))
            .cloned()
            .collect()
    }

    pub fn solve() -> HashSet<(i32, i32)> {
        let all_pairs = get_all_pairs();
        let s1 = sieve(&all_pairs, satisfy_s1);
        let s2 = sieve(&s1, |(i, j)| satisfy_s2((i, j), &s1));
        let s3 = sieve(&s2, |(i, j)| satisfy_s3((i, j), &s2));
        s3
    }
}

fn main() {
    let ans = solution::solve();
    print!("{:?}", ans);
}
