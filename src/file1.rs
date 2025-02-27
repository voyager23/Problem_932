// Filename: file1.rs

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
        println!("lhs: {}  rhs: {} ", lhs, rhs);
        // TODO this should be the concatenation of lhs&rhs
        while rhs > 0 {
            lhs *= 10;
            lhs = lhs + rhs/10;
            rhs /= 10;
        }
        println!("concat {}", lhs);
        if lhs == n {
            flag = true;
            break;
        }
        
        d *= 10;
        if d > sq { break; }  // Ensuring termination
    }
    flag
}

pub fn concat(n:u32) -> u32 {

    let sn = n.to_string();
    let digits = sn.len();

    let snn = (n*n).to_string();
    let length = snn.len();

    let z = length-digits;

    let q = &snn[z..];  // right hand side string
    let q_number: u32 = q.parse().expect("Not a valid number");   // rhs convert to integer
    let p = &snn[..z];  // left hand side string
    let p_number: u32 = p.parse().expect("Not a valid number");   // lhs convert to integer
    let pq_number = p_number + q_number;

    let cat_str = q_number.to_string(); // rhs back to string
    let result = snn[0..z].to_owned() + &cat_str;   // concatenate
    let number: u32 = result.parse().expect("Not a valid number"); 

    if number == pq_number * pq_number
    {
        println!("{} is a solution", number );
    }

    number

}
//-----------------------------------------------------------
/*    let mut ab : u32 = n;
    let s : String = ab.to_string();
    let pq = &s[d..];
    let number: u32 = pq.parse().expect("Not a valid number"); 
    ab = (ab/100)*10 + number;
    // println!("{}",ab);
    ab*/


pub fn digits(lo:u32, hi:u32) -> Vec<(u32,u32,u32)>{
    let mut limits : Vec<(u32,u32,u32)> = vec![];
    for digits in lo..hi+1{
        let first : u32 = u32::pow(10,digits-1) + 1;
        let end = u32::pow(10,digits);
        println!("digits:{}\tfirst:{}  end:{}", digits, first, end);
        limits.push((first,end,digits));
    }
    limits
}


/*pub fn concat(n:u32, d:usize) -> u32 {
    let mut ab : u32 = n;
    let s : String = ab.to_string();
    let pq = &s[d..];
    let number: u32 = pq.parse().expect("Not a valid number");
    ab = (ab/100)*10 + number;
    // println!("{}",ab);
    ab
}*/