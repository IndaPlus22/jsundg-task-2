/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */
// use array2d::Array2D;
use std::io;
/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed read");

    
    let input_vec: Vec<usize> = input
    .trim()
    .split_whitespace()
    .map(|x| x.parse().expect("parse error"))
    .collect();
    
    
    for mut _i in 1..(input_vec[0]+1){
        for mut _j in 1..(input_vec[1]+1){
            let mut _distance = distance_to_edge(_i,_j,input_vec[0],input_vec[1]);
            
            if _distance > 9{
                print!(".");
            }
            else{
                print!("{}",_distance);
            }
        }
        print!("\n");
    }

   
    // get input lines as strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    // Vector not necessary, see example solution.

    /* add code here ... */
    
    //println!("Print to standard output.");
}
fn distance_to_edge(mut x:usize,mut y:usize, r:usize,k:usize) -> usize{
    if x > (r / 2) { 
            x = r - x + 1;
        }
       
    if y > (k / 2){
            y = k - y + 1;
        }
       
    if x <= y {
            return x;
        }
       
    else {
            return y; 
        }
        
}
   
   