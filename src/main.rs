#[allow(dead_code)]
mod file1;

fn main() {
    // Found sequence A248353 in OEIS. Refered to as kaprekar Numbers.
    //  n such that n=q+r and n^2=q*10^m+r, for some m >= 1, q>=0 and 0<=r<10^m.
    let mut _foo : bool;
    for n in 99..100 {
         _foo = file1::twenty_twentyfive(n);
         if _foo {
            println!("{0} {1} {2}", n, n*n, _foo);
        }
    }
}
