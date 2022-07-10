use std::io::Read;

pub fn read_file(_v:Vec<String>) -> Vec<String>{
    let mut file_content:Vec<String>=Vec::new();
    
    for name in _v{
    let mut f_name=String::from("./src/resumes/");
        f_name.push_str(&name);
        let mut buff:String=String::new();
        let mut  file=std::fs::File::open(f_name).unwrap();
        
       let _res= file.read_to_string(&mut buff);
     
       file_content.push(buff);
       

         


    }
    return file_content;
}