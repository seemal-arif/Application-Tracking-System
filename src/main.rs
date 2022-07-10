#[warn(unused_imports)]
use std::io::stdin;
use std::{thread, time, io::Read};
use std::fs;
use std::io::Write;
use std::io;


mod get_file_name;
mod read_file;
mod get_key_words;
mod find_match;
mod filter;

fn verify_login(buffer:String,name:String,num:u32)->bool{

    let mut check_user_match:u32=0;//to check validity.
    let  mut ret:bool=true; 
    for item in buffer.split('\n'){ // read line by line .
        let mut count:u32=0;
        for val in item.split('/'){
            count=count+1;  //to read match username with username.
            if count==num{
                let mut check:bool=val.eq(&name);
                if check==true{   //username match
                    check_user_match=check_user_match+1;        
                    

                }
            }
           
        }
       
        
    }
    if check_user_match==0 {
        ret=false;
    }
    return ret;
}


fn main() {
    
    print! ("\x1B[2J\x1B[1;1H"); 
    println!("\n");
    println!("        Welcome to the portal        ");
  
    println!("\n");
    print!("Do you want to login  yes or no ?  ");
    io::stdout().flush().unwrap();
    
    let mut f:String=String::new();
    let _r=stdin().read_line(&mut f).unwrap();
    let com:String=String::from("yes");
    let check:bool=com.eq(&f.trim());
  
    if check==true {  // if user want to login
       print! ("\x1B[2J\x1B[1;1H"); //clear the terminal and move curser to first row and first column.

       println!("Preparing System for you ");  
       print! ("\x1B[2J\x1B[1;1H");

       let ten_millis = time::Duration::from_millis(300);
       let now = time::Instant::now();
       thread::sleep(ten_millis);
       assert!(now.elapsed() >= ten_millis);

       println!("Preparing System for you. ");  
       print! ("\x1B[2J\x1B[1;1H");

       let ten_millis = time::Duration::from_millis(300);
       let now = time::Instant::now();
       thread::sleep(ten_millis);
       assert!(now.elapsed() >= ten_millis);
    
       println!("Preparing System for you.. ");  
       print! ("\x1B[2J\x1B[1;1H");

       let ten_millis = time::Duration::from_millis(300);
       let now = time::Instant::now();
       thread::sleep(ten_millis);
       assert!(now.elapsed() >= ten_millis);

       println!("Preparing System for you... "); 
       
       let ten_millis = time::Duration::from_millis(300);
       let now = time::Instant::now();
       thread::sleep(ten_millis);
       assert!(now.elapsed() >= ten_millis);

       print! ("\x1B[2J\x1B[1;1H");
      
       println!("There you go ");
       print! ("\x1B[2J\x1B[1;1H");

       let ten_millis = time::Duration::from_millis(300);
       let now = time::Instant::now();
       thread::sleep(ten_millis);
       assert!(now.elapsed() >= ten_millis);

       println!("There you go. ");
       print! ("\x1B[2J\x1B[1;1H");

       let ten_millis = time::Duration::from_millis(300);
       let now = time::Instant::now();
       thread::sleep(ten_millis);
       assert!(now.elapsed() >= ten_millis);

       println!("There you go.. ");
       print! ("\x1B[2J\x1B[1;1H");

       let ten_millis = time::Duration::from_millis(300);
       let now = time::Instant::now();
       thread::sleep(ten_millis);
       assert!(now.elapsed() >= ten_millis);

       println!("There you go... ");
       

       let ten_millis = time::Duration::from_millis(300);
       let now = time::Instant::now();
       thread::sleep(ten_millis);
       assert!(now.elapsed() >= ten_millis);

       print! ("\x1B[2J\x1B[1;1H"); 

       
       
       
            loop{     // username name is wrong you can enter again .
                println!("\n");
                println!("               LOGIN PAGE               ");
                println!("\n");

        
                print!("Enter Username : ");
                io::stdout().flush().unwrap();

        
                let mut username:String=String::new();
                let _fd=stdin().read_line(&mut username).unwrap();
                let name:String=String::from(username.trim());

                let mut file=fs::File::open("./src/pass.txt").unwrap();
                let mut buffer:String=String::new();
                let _res=file.read_to_string(&mut buffer).unwrap();
                // println!("{}",buffer);
       
                let ans:bool= verify_login(buffer.clone(), name, 1);//to verify username.

                 
                print! ("\x1B[2J\x1B[1;1H");
                println!("\n");
                println!("\n");
                println!("Verifying Username");

                let ten_millis = time::Duration::from_millis(500);
                let now = time::Instant::now();
                thread::sleep(ten_millis);
                assert!(now.elapsed() >= ten_millis);

                print! ("\x1B[2J\x1B[1;1H");
                println!("\n");
                println!("\n");
                println!("Verifying Username.");

                let ten_millis = time::Duration::from_millis(500);
                let now = time::Instant::now();
                thread::sleep(ten_millis);
                assert!(now.elapsed() >= ten_millis);

                print! ("\x1B[2J\x1B[1;1H");
                println!("\n");
                println!("\n");
                println!("Verifying Username..");

                let ten_millis = time::Duration::from_millis(500);
                let now = time::Instant::now();
                thread::sleep(ten_millis);
                assert!(now.elapsed() >= ten_millis);

                print! ("\x1B[2J\x1B[1;1H");
                println!("\n");
                println!("\n");
                println!("Verifying Username...");

                let ten_millis = time::Duration::from_millis(500);
                let now = time::Instant::now();
                thread::sleep(ten_millis);
                assert!(now.elapsed() >= ten_millis);

                let mut count:u8=0;  //check incorrect password entry count
                let mut iteration:u8=3;
                if ans==true{
                   print! ("\x1B[2J\x1B[1;1H");
                   println!("\n");
                   println!("\n");
                   println!(" Username  Verified :)");

                   let ten_millis = time::Duration::from_millis(800);
                   let now = time::Instant::now();
                   thread::sleep(ten_millis);
                   assert!(now.elapsed() >= ten_millis);

                    
                   while iteration!=0 {  //verify and user can enter invalid password only 3 times
                      print! ("\x1B[2J\x1B[1;1H");
                      println!("\n");
                      
                      println!("               LOGIN PAGE               ");
                      println!("\n");
                     iteration=iteration-1;
                     print!("Enter Password : ");
                     io::stdout().flush().unwrap();
                     let mut password:String=String::new();
                     let _rr=stdin().read_line(&mut password).unwrap();
                     let mut pass:String=String::from(password.trim());
                     let verify:bool=verify_login(buffer.clone(),pass, 2);

                     print! ("\x1B[2J\x1B[1;1H");
                     println!("\n");
                     println!("\n");
                     println!("Verifying Password");

                     let ten_millis = time::Duration::from_millis(500);
                     let now = time::Instant::now();
                     thread::sleep(ten_millis);
                     assert!(now.elapsed() >= ten_millis);

                     print! ("\x1B[2J\x1B[1;1H");
                     println!("\n");
                     println!("\n");
                     println!("Verifying Password.");

                     let ten_millis = time::Duration::from_millis(500);
                     let now = time::Instant::now();
                     thread::sleep(ten_millis);
                     assert!(now.elapsed() >= ten_millis);

                     print! ("\x1B[2J\x1B[1;1H");
                     println!("\n");
                     println!("\n");
                     println!("Verifying Password..");

                     let ten_millis = time::Duration::from_millis(500);
                     let now = time::Instant::now();
                     thread::sleep(ten_millis);
                     assert!(now.elapsed() >= ten_millis);

                     print! ("\x1B[2J\x1B[1;1H");
                     println!("\n");
                     println!("\n");
                     println!("Verifying Password...");

                     let ten_millis = time::Duration::from_millis(500);
                     let now = time::Instant::now();
                     thread::sleep(ten_millis);
                     assert!(now.elapsed() >= ten_millis);
                      
                     if verify==true {   // password is verified
                          
                          print! ("\x1B[2J\x1B[1;1H");
                          println!("\n");
                          println!("\n");
                          println!(" Password Verified :) ");

                          let ten_millis = time::Duration::from_millis(800);
                          let now = time::Instant::now();
                          thread::sleep(ten_millis);
                          assert!(now.elapsed() >= ten_millis);

                          print! ("\x1B[2J\x1B[1;1H");
                          println!("\n");
                          println!("  Welcome to Application Tracking System ");
                          let current_directory:String="./src/resumes/".to_string();
                          let _v:Vec<String>=get_file_name::get_file_name(current_directory);//get file names in given directory.
                          let _content:Vec<String>=read_file::read_file(_v.clone());//return content of each file in current directory.
                          let _key_words:Vec<String>=get_key_words::get_key_words();// get key word from recruiter.
                          let  find_count:Vec<u32>=find_match::find_match(_content.clone(),_key_words.clone());//
    
                          let mut size:u32=_v.len() as u32;
                          let  total_keyword:f64=_key_words.len() as f64;
                          let mut length:usize=_v.len();
                          filter::filter(find_count, size, total_keyword,_v);//Filter outs the qualified resumes.
                          return;


                        }
                      else {
                          count=count+1;
                          print! ("\x1B[2J\x1B[1;1H");
                          println!("\n");
                          println!("\n");
                          println!("   Incorrect Password ");

                          let ten_millis = time::Duration::from_millis(1000);
                          let now = time::Instant::now();
                          thread::sleep(ten_millis);
                          assert!(now.elapsed() >= ten_millis);
                          print! ("\x1B[2J\x1B[1;1H");
                         //   println!("Invalid password");
                         //   print!("Again enter ");
                         //   io::stdout().flush().unwrap();
                        }
                    }
                   if count==5{ 
                      print! ("\x1B[2J\x1B[1;1H");
                      println!("You Have Already Entered Incorrect Password Five Times");
                      println!("\n");
                      return;
                    }

                }
                else{ //if username do not match.
                    print! ("\x1B[2J\x1B[1;1H");
                    println!("\n");
                    println!("\n");
                    println!("Incorrect Username ");

                    let ten_millis = time::Duration::from_millis(1000);
                    let now = time::Instant::now();
                    thread::sleep(ten_millis);
                    assert!(now.elapsed() >= ten_millis);

                    print! ("\x1B[2J\x1B[1;1H");



                    // prin!("Again enter ");
                    // io::stdout().flush().unwrap();
                }
                

            }

       
    }
    else {  //if user do not want to login 
     //std::process::exit(0x0100);
      return;
    }

    
}
    









