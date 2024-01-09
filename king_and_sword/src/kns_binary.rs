/*
    The general idea is that this can be solved mathematically by subtracting
    the highest power of 2 possible (while still being positive), then doubling
    the number and adding one.

    This can be done by manipulating the binary of the number.
    First, remove the left-most high value(1). This effectively subtracts the
    highest power of 2 possible (2^x). Then, push all bits one to the left which
    effectively multiplies the remaining number by 2. This would normally be done by
    appending a 0 but if you just append by 1, it's the same as combining both steps
    or saying 2x+1. The answer is then converted back from string to u32 and returned.

    EG: integer 10
    Done in integer math:
        highest power of 2 that can be subracted is 8 leaving 2 as our working number
        double working number of 2 to get 4 as new working number
        add 1 to 4 to get solution of 5
    Done in binary manipulation: 
        10 expressed in binary is 0b1010 -> 10_u32
        remove the leftmost high bit (equivalent to subtracting highest power of 2): 0b010 -> 2_u32
        append a 1 to the end (equivalent to multiplying by 2 and adding 1): 0b00101 -> 5_u32
        This is the solution. 5
    Done manually:
        1 kills 2, 3 kills 4, 5 kills 6, 7 kills 8, 9 kills 10  alive: [1,3,5,7,9]
        1 kills 3, 5 kills 7, 9 kills 1                         alive: [5, 9]
        5 kills 9.                                              alive: [5]
        5 is the survivor.
*/
pub fn get_last_man_standing( num: u32 ) -> u32 {
    let mut binary_string: String = format!("{num:b}");
    binary_string.remove(0); // subtract the highest power of 2 (drop the left-most 1 from the binary string)
    binary_string.push('1'); // multiply by 2 and add 1 (multiply by 2 would normally append a 0 but combining adding 1 gives appending a 1)

    let binary_answer: u32 = u32::from_str_radix(&binary_string, 2).unwrap(); //convert the binary string back to u32

    binary_answer
}