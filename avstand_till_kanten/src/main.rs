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
    
    let  rows: usize= input_vec[0];
    let  columns: usize = input_vec[1];
    
   
    for mut _i in 1..(rows+1){
        for mut _j in 1..(columns+1){
            if distance_to_edge(_i,_j,rows,columns) > 9{
                print!(".");
            }else{
                print!("{}",distance_to_edge(_i,_j,rows,columns))
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
    let mut _distance: usize = 0;
        if x > (r / 2) {
            x = r - x + 1;
        }
        if y > (k / 2) {
            y = k - y + 1;
        }
        if x <= y { 
        _distance = x;
        return _distance;
        }else{
        _distance = y;
        return _distance;
        }
        
    }
   
   