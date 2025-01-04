pub fn tempurature_conversion(value: f64, unit: &str) -> f64 {
    if unit == "c" {
        return value * 9.0 / 5.0 + 32.0;
    } else if unit == "f" {
        return (value - 32.0) * 5.0 / 9.0;
    } else {
        return 0.0;
    }
}

pub fn nth_fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
    }
}
