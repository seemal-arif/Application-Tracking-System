use std::fs;


pub fn get_file_name(directory:String)->Vec<String> {
    let paths=fs::read_dir(directory).unwrap();
    let mut v:Vec<String>=Vec::new();
       
    for path in paths{
        
        let  mut file_name:String=String::new();
        match path {
            Ok(l)=>  file_name = l.path().file_name().unwrap().to_string_lossy().into_owned(),
            Err(_)=>println!("error"),
        } 
        v.push(file_name);
       
        
    }
    return v;
   

    
    

}





