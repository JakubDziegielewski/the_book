fn main() {
   //fibo(10);
    let res = euklides(3918848, 1653264);
    println!("{res}");
}


fn _fibo(lentgth: u32){
    let mut left: u128 = 0;
    let mut rigth: u128 = 1;
    let mut temp: u128;
    println!("0: 0");
    println!("1: 1");
    for i in 2..lentgth{
        temp = left + rigth;
        left = rigth;
        rigth = temp;
        println!("{i}: {temp}");
    }
}
fn euklides(mut a:u32, mut b:u32) -> u32{
    while b>0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
