fn main() {
    let mut a: u128 = 1;
    let mut b: u128 = 2;
    let mut sum: u128 = 0;
    for number in 1..100 {
        println!("{:<5} {}",number,a);
        if a >= 4000000 {
            break;
        }
        if a % 2 == 0 {
            sum+=a;
        }
        let oa = a;
        a = b;
        b+=oa;
    }
    println!("Answer: {}",sum);
}
