use std::io::stdin;
use std::io::Write;
use std::io;

use std::{thread, time, io::Read};

pub fn get_key_words()->Vec<String>{

    let ten_millis = time::Duration::from_millis(800);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);

    println!("\n");
    print!("Enter Number of words to match : "); 
    io::stdout().flush().unwrap();  
    let mut number:String=String::new();
    let _r=stdin().read_line(&mut number).unwrap();
    let mut _num:u32=0;
    match number.trim().parse::<u32>(){   //converting string into integer
        Ok(l)=>_num=l,
        Err(_)=>_num=0,//indicating error
    }  
    
    println!("Enter words : ");
  
    let mut words_to_find:Vec<String>=Vec::new();

    while _num!=0{

        let mut key_words:String=String::new();
        let _res=stdin().read_line(&mut key_words).unwrap();
        words_to_find.push(key_words);
        _num=_num-1;
       
    }
   return words_to_find;
  

}