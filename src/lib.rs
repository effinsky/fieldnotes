mod app;
mod domain;

pub fn fact(n: i32) -> i32 {
    fact_aux(n, 1)
}

fn fact_aux(n: i32, result: i32) -> i32 {
    match n {
        0 | 1 => result,
        _ => fact_aux(n - 1, result * n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(5), 120);
        assert_eq!(fact(10), 3628800);
    }
}
