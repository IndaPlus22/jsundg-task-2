
/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */
 use std::convert::TryInto;
 use std::io;
 
 /// Kattis calls main function to run your solution.
 fn main() {
     // get standard input stream
     let mut input_length = String::new();
     let mut input_list = String::new();

     io::stdin().read_line(&mut input_length).expect("Failed read on length");
     io::stdin().read_line(&mut input_list).expect("Failed read on list");
    // didnt understand how the template worked so changed to my own method
     // get input lines as strings
     // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut length: u32 = input_length
    .trim()
    .parse().expect("Could not parse length");
    
    if length % 2 == 1{
        length += 1;
    }
   
    
        //https://stackoverflow.com/questions/26536871/how-can-i-convert-a-string-of-numbers-to-an-array-or-vector-of-integers-in-rust
        let mut num_list: Vec<u32> = input_list
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("parse error"))
        .collect();
        num_list.sort();
        num_list.reverse();
        
      

        println!("{}", sum_of_list(num_list,length));
    }
   
    fn sum_of_list(mut x: Vec<u32>, length: u32) -> u32 {
            x.truncate((length / 2).try_into().unwrap());
            x.iter().sum()
       

    }
    
       
     //eprintln!("Kattis skips this comment!");
     //println!("Print to standard output.");
 