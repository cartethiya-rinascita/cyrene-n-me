use std::collections::HashMap;

pub struct FibonacciCalculator {
    cache: HashMap<u64, u64>,
}

impl FibonacciCalculator {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn calculate(&mut self, n: u64) -> u64 {
        if let Some(&result) = self.cache.get(&n) {
            return result;
        }

        let result = match n {
            0 => 0,
            1 => 1,
            _ => {
                let a = self.calculate(n - 2);
                let b = self.calculate(n - 1);
                a + b
            }
        };

        self.cache.insert(n, result);
        result
    }
}
