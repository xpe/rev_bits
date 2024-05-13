fn main() {
    {
        use rev_bits::u32::reverse;
        let x: u32 = 0xF0FFA000;
        let y = reverse(x, 8..16);
        println!("## u32");
        println!("original: {:08X}", x);
        println!(" changed: {}", "....xx..");
        println!("reversed: {:08X}", y);
    }
    {
        use rev_bits::u64::reverse;
        let x: u64 = 0xF012341234FFA000;
        let y = reverse(x, 8..16);
        println!("\n## u64");
        println!("original: {:016X}", x);
        println!(" changed: {}", "............xx..");
        println!("reversed: {:016X}", y);
    }
}
