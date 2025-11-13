use std::collections::HashMap;

use crate::utils::fibonacci;

pub fn run_fibonacci() {
    let n: u64 = 65;
    let mut cache = HashMap::new();

    for i in 0..n {
        let res = fibonacci::fibonacci_memo(i, &mut cache);
        print!("{res} ");
    }
}
