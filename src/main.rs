mod prob_1;

fn main() {
}
#[test]
fn test_solution() {
    assert_eq!(prob_1::solution(1000), 233168);
    assert_eq!(prob_1::solution(10), 23);
    assert_eq!(prob_1::solution(5), 3);
}