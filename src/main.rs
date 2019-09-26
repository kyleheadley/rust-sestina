#![allow(non_camel_case_types)]

#[macro_use]
mod parse;

fn main() {
  narrate![start
    type a = AND {b:String,c:u32}
    type b = OR {five, six, seven}
  ];


  let the_a : a = a{b:"one".into(), c:5};
  let the_b : b = b::five;
  println!("{:?},{:?}", the_a, the_b);
}
