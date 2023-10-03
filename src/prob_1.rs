// https://projecteuler.net/problem=1
pub fn solution(upto: i32) -> i32 {
    // let mut sum = 0;
    // for num in 1..1000 {
    //     if num % 3 == 0 || num % 5 == 0 {
    //         sum += num;
    //     }
    // }
    // sum
    (1..upto).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

