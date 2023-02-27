macro_rules! five_times {
    ($x:expr) => {
        for i in 0..5 {
            $x(i);
        }
    };
}

fn main() {
    five_times!(|i| {
        println!("The value of i is {}", i);
    });
}
