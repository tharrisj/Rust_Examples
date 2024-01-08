/*
    Simple, brute-force implementation.
    Create an array of num elements all set to 1 representing alive state
    This is technically from 0 to num-1 but the offset at the end fixes
    the survivor. Modulo is used lazily so we don't have to restart the value
    of i every time we loop through.
    i represents the current killer
    loop through to find the next killer
    killed_idx represents the target to be killed
    loop through to find the next target to be killed
    if killer is looking at themselves to be killed, break the loop as we've found the survivor
    return the index of i + 1.
*/
pub fn get_last_man_standing( num: u32 ) -> u32 {
    let mut persons: Vec<u8> = vec![1_u8; num as usize];
    // println!("{:?}", persons);

    let mut i: u32 = 0;
    loop {
        // iterate through and find next alive person to be killer
        while persons[ (i % num) as usize ] != 1 {
            i += 1;
        }

        let mut killed_idx: u32 = i+1 % num;

        // iterate through and find next alive person to be killed
        while persons[ ( killed_idx % num) as usize ] != 1 {
            killed_idx += 1;
        }

        // killer is looking at themselves. break.
        if i % num == killed_idx % num  {
            break;
        }
        
        persons[ (killed_idx % num) as usize ] = 0; // kill the array spot
        i = killed_idx + 1;
    }
    
    let survivor: u32 = (i % num) + 1;

    survivor
}