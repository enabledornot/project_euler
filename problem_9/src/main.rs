// solved on paper
fn generate_triple(m: i32, n: i32) -> [i32;3] {
    let a = m*m - n*n;
    let b = 2*n*m;
    let c = n*n + m*m;
    return [a, b, c];
}
fn main() {
    for i in 1..10 {
        print!("{}:",i);
        for j in i+1..10 {
            print!("{}-{:?} ",j,generate_triple(j,i));
        }
        println!("");
    }
}
