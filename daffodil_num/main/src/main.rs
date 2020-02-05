use tlib::math;
fn main() {
    let mut num = (0,0,0);
    for i in 100..1000 {
        num.0 = math::sqrt(i % 10, 3);
        num.1 = math::sqrt(i / 10 % 10, 3);
        num.2 = math::sqrt(i / 100 % 10, 3);
        if i == num.0 + num.1 + num.2 {
            println!("{} ",i);
        }
    }
}