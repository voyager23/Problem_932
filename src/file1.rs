pub fn func1(){

    println!("module func1");
    for n in 1..100 {
        print!("{} ", n);
        println!("{}", n*n);
    }

}

pub fn func2(n: u32){
    println!("module func2");
    let mut d = 10;
    loop {
        print!("{}  ", n % d);
        d = d*10;
        if n % d == n { break;}; 
    }
}

pub fn twenty_twentyfive(n: u32) -> bool {
    let mut d = 10;
    let sq = n * n;
    let mut flag = false;

    loop {
        let mut lhs = sq / d;
        let mut rhs = sq % d;
        println!("lhs: {}  rhs: {}  sum: {}", lhs, rhs, lhs + rhs);
        // TODO this should be the concatenation of lhs&rhs
        while rhs > 0 {
            lhs *= 10;
            dbg!(lhs);
            dbg!(rhs);
            lhs = lhs + rhs/10;
            rhs %= 10;
        }

        if lhs == n {
            flag = true;
            break;
        }
        
        d *= 10;
        if d > sq { break; }  // Ensuring termination
    }
    flag
}
