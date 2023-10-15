fn main() {
    let ans = quick_sum(3,999) + quick_sum(5,999) - quick_sum(15,999);
    println!("Answer: {}",ans);
}

fn quick_sum(split: i32, max: i32) -> i32 {
    let count = max / split;
    split*(count*(count+1))/2
}