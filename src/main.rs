#[allow(dead_code)]
mod file1;

fn main() 
{
    // Found sequence A248353 in OEIS. Refered to as kaprekar Numbers.
    //  n such that n=q+r and n^2=q*10^m+r, for some m >= 1, q>=0 and 0<=r<10^m.

    let _limits = file1::digits(1,2);
    let mut _t : (u32,u32) = (42,42);
    for t in  _limits 
    { 
       for x in t.0 .. t.1 {
            println!("{}",x);
       }
    }

    //let x : u32 = file1::concat(9801);
    //println!("9801 -> {}", file1::concat(9801) );
}



/*
Scratch pad area
let mut _limits : Vec<(u32,u32)> = vec![];
    let mut _foo : bool;
    for n in 99..100 {
         _foo = file1::twenty_twentyfive(n);
         if _foo {
            println!("{0} {1} {2}", n, n*n, _foo);
        }
    }
*/