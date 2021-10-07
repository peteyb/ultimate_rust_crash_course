const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    let (mut missiles2, ready2): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Set2: Firing {} of my {} missiles...", ready2, missiles2);

    missiles2 = missiles2 - ready2;
    println!("{} missiles2 left", missiles2);

    println!("{} in print line", missiles - ready);

    READY_AMOUNT = 2;
}
