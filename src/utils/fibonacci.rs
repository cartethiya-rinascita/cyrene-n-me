use std::collections::HashMap;

pub fn fibonacci_memo(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }

    let result = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_memo(n - 2, cache) + fibonacci_memo(n - 1, cache),
    };

    cache.insert(n, result);
    result
}
