// Filename: file2.rs
#[allow(dead_code)]

pub fn func2(){
    println!("module func2");
    for n in 1..10 {
        print!("{} ", n);
        println!("{}", n*n);
    }
}

pub fn test2025(n:u32)
{
    let mut _digits = 0;
    let mut a = n;
    while a > 0
    {
        a /= 10;
        _digits += 1;
    }
    let nn :u32 = n*n;
    let a_num = nn / u32::pow(10,_digits);
    let mut b_num = nn % u32::pow(10,_digits);
    print!("{} {} {} {}", n, nn, a_num, b_num);
    // now need to calc a_num + b_num
    let a_sum_b = a_num + b_num;    // need to square this number
    let mut a_cat_b = a_num;
    let mut v: Vec<u32> = vec![];
    while b_num > 0
    {
        v.push(b_num % 10);
        b_num /= 10;
    }
    // v has digits in reverse order
    while v.is_empty() == false
    {
        a_cat_b *= 10;
        let mm : u32 = *v.last().unwrap();
        a_cat_b = a_cat_b + mm;
        v.truncate(v.len() - 1);
    }
    println!("  sum:{} sum_sqrd:{} cat:{}", a_sum_b, a_sum_b*a_sum_b, a_cat_b);
    if a_sum_b*a_sum_b == a_cat_b
    {
        println!("Solution!");
    }
}
