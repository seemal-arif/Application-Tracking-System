



pub fn find_match(_content:Vec<String>,_key_words:Vec<String>)->Vec<u32>{
    let mut count:u32=0;
    let mut count_occurence:Vec<u32>=Vec::new();
    
    for item in _content{   //give content of each file one by one. 
      
       
        for l in &_key_words{ 
           
            for i  in item.split_whitespace(){
               
                let a=i.to_string();
                let mut c=l.clone();
                let mut b=c.trim();                
                
                let check:bool= a.to_lowercase().eq(&b.to_lowercase());
                
               if check==true {    // if word match 
                count=count+1;
                break;
               }
            }
        }
       
        count_occurence.push(count);
        count=0;

    }
    return count_occurence;
   


}