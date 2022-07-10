use std::io::stdin;
use std::io::Write;
use std::io;
#[derive(Debug)]
struct alpa{
    count:u32,
    file_name:String
    
}

pub fn filter(find_count:Vec<u32>,mut size:u32, total_keyword:f64,_v:Vec<String>) {
    let mut percentage:Vec<u32>=Vec::new();
   
    // for u in find_count.clone(){
    //     println!(" count are {} ",u);
    // }
    // println!("\n");

    for count in find_count{     //converting count in percentage form
        let num= count as f64;
        let num=num/total_keyword;
        let num=num*100.0;
        let num:u32=num as u32;
        percentage.push(num);

    }
    // println!("\n");
    // for u in percentage.clone(){
    //     println!(" count are {} ",u);
    // }
    

    
    let mut i:usize=0;
    let mut arr:Vec<alpa>=Vec::new();
    let mut length:usize=_v.len();
    let mut strr:&String=&String::new();
    let mut n:&u32=&0;
    let mut sum:u32=0;


    while length!=0  { // build a vector of struct and struct contains filename and count of match

        let  q= _v.get(i);

        match q {
            Some(j)=>strr=j,
            _=> ()
        }

       // println!("strr {} ",strr);

        let w=percentage.get(i);
        match w {
            Some(l)=>n=l,
            _=>()
        }
        let mut _str=String::from(strr.clone().as_str());
        
        arr.push(alpa{count:*n,file_name:_str.clone()});

       // println!("vector of struct is -{:?}-",arr[i]);

        i=i+1;
        length=length-1;
        sum=sum+*n;

    }
    
    let  mut avg:u32=sum/size;
    println!("Average percentage match in resumes is  {}%",avg);
    print!("Enter Accepted percentage match : ");
    io::stdout().flush().unwrap();
    let mut s:String=String::new();
    let _res=stdin().read_line(&mut s).unwrap();
    // println!("{}",s);
    let mut range:u32=0;
    match s.trim().parse::<u32>(){
        Ok(i)=>range=i,
        Err(_)=>()
    };
    // println!("the  range is {} ",range);
    let mut val:u32=0;
    let arr_itr=&arr[..];
    for item in arr_itr {
        if item.count>=range {
            val=val+1;
        }

    
    }
    if  val>0 {
       println!("Qualified Resumes are : ");
       for item in arr_itr {
          if item.count>=range
           {
              println!(" {}",item.file_name);

           }  
        }
    }
    
    else{
        println!("No one has Qualified ......... ");
    }

}



