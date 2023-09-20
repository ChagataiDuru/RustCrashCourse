const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    println!("Hello, Rust!");

    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    let dummy: i32;

    println!("Firing {} of my {} missiles...", ready, missiles);

    /*missiles = missiles - ready;

    println!("{} missiles left", missiles);*/

    println!("{} missiles left", missiles-ready);


}
