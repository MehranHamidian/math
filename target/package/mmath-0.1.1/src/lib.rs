// to_binary32()
//this function get and i32 decimal number and convert it to 32-bits binary number.
//this function return a immutable rust array contatin 0 and 1.
//the length of the array is 32.
pub fn to_binary32(mut number: i32) -> [u8; 32] {
    let mut mut_binary: [u8; 32] = [0; 32]; //define a mut array with 32 0bits
    let mut index: usize = 31; // define last index of array
    if number >= 0 {
        loop{
            mut_binary[index] = (number % 2) as u8;
            number/=2;
            index-=1;
            if number == 0 {
                break
            }
        }
    }else {
        //make number positive
        number = - number;
        loop{
            mut_binary[index] = (number % 2) as u8;
            number/=2;
            index-=1;
            if number == 0 {
                break
            }
        }
        //invert the bits
        let mut indexc: usize = 0;
        while indexc < 32 {
            if mut_binary[indexc] == 0{
                mut_binary[indexc] = 1;
            }else{
                mut_binary[indexc] = 0
            }
            indexc += 1;
        }
        // add one
        mut_binary[31] = 1;
    }
    let binary: [u8; 32] = mut_binary; // make it immutable
    binary //and finally return it
}

//trail_division
//this function take a u64 number and define the number is prime or not
//this function return false for all the numbers less than 2
pub fn trail_division(number: u64) -> bool {
    let mut boolean = true;
    if number < 2 {
        boolean = false;
    }else {
        let square_root: u64 = (number as f64).sqrt().floor() as u64;
        for i in 2..=square_root{
            if number % i == 0{
                boolean = false;
            }
        }
    }
    boolean
}
