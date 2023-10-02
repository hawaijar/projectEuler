// https://projecteuler.net/problem=1
fn solution() -> i32 {
    let mut sum = 0;
    for num in 1..1000 {
        if num % 3 == 0 || num % 5 == 0 {
            sum += num;
        }
    }
    sum
}

#[test]
fn test_solution() {
    assert_eq!(solution(), 233168);
}