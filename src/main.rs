use rev_bits::u32::reverse;

fn main() {
    let x: u32 = 0xF0FFA000;
    let y = reverse(x, 8..16);
    println!("original: {:08X}", x);
    println!(" changed: {}", "....xx..");
    println!("reversed: {:08X}", y);
}
