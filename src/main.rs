#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt;

struct BigInt {
    number: String,
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl BigInt {
    fn new() -> BigInt{
        BigInt{number: "".to_string()}
    }
    fn set(&mut self, x: String){
        self.number = x;
    }
    fn add(n: BigInt, m: BigInt) -> BigInt{
        fn aux(min: Vec<char>, max: Vec<char>) -> String{
            let mut carry: u32 = 0;
            let mut ans: String = "".to_string();
            let mut i = 0;
            while i < min.len(){
                let x = carry + min[i].to_digit(10).unwrap() + max[i].to_digit(10).unwrap();
                ans.push_str(&(x.to_string())[..]);
                carry = x/10;
                i += 1;
            }
            while i < max.len(){
                ans.push(max[i]);
                i += 1;
            }
            ans.chars().rev().collect::<String>()
        }
        let Nlist: Vec<char> = n.number.chars().rev().collect();
        let Mlist: Vec<char> = m.number.chars().rev().collect();
        let mut ans = BigInt::new();
        if Nlist.len() > Mlist.len(){
            ans.set(aux(Mlist ,Nlist));
        }else {
            ans.set(aux(Nlist, Mlist));
        }
        ans
    }
}

fn f(a: &str){

}

fn main() {
    let n: BigInt = BigInt{number: "0".to_string()};
    let m: BigInt = BigInt{number: "213".to_string()};
    println!("{}", BigInt::add(n, m));
}
