use std::{env, path::Path, fs::{self, metadata}};



fn pwd() {

    let dir = env::current_dir().unwrap();
            println!("{}",dir.display())
}

fn echo(args: Vec<String>) {
    if args[2].as_str()=="-n"{
        print!("{}", env::args().skip(3).collect::<Vec<_>>().join(" "))
    }else{
        println!("{}", env::args().skip(2).collect::<Vec<_>>().join(" "))
    } 
}

fn cat (args: Vec<String>){
    
    for i in 2..args.len(){
        
    let newpath = Path::new(&args[i]);
    let contents = fs::read_to_string(newpath);
        match contents{
            Ok(contents)=>print!("{}", contents),
            Err(_)=>std::process::exit(-20)
        }  
    } 

}


fn mkdir(args : Vec<String>){
    for i in 2 .. args.len(){
        let path = Path::new(&args[i]);
    match fs::create_dir_all(path){
        Ok(_)=>{},
        Err(_)=>std::process::exit(226)
    }
}
    }


fn mv(args : Vec<String>){
    let path = Path::new(&args[2]);
    let b = std::path::Path::new(path).exists();

   if b==true{
    let path2 = Path::new(&args[3]);
    match fs::rename(path, path2){
        Ok(_)=>{},
        Err(_)=>std::process::exit(-40)
    }
   }else {
    std::process::exit(-40)
   }

}    

fn ln(args : Vec<String>){
    if args[2].as_str()=="-s"||args[2].as_str()=="--symbolic" {
        let from = Path::new(&args[3]);
        let to = Path::new(&args[4]);

        match std::os::windows::fs::symlink_file(from, to){
            Ok(_)=>{},
            Err(_)=>std::process::exit(-50)
        }
    } else {
        let from = Path::new(&args[2]);
        let to = Path::new(&args[3]);

        match fs::hard_link(from, to){
            Ok(_)=>{},
            Err(_)=>std::process::exit(-50)
        }
    }
}

fn rmdir(args : Vec<String>){
    for i in 2 .. args.len(){
        let path = Path::new(&args[i]);
        let md = metadata(path);
    match md{
        Ok(md)=>{
                           if md.is_dir(){
                            match fs::remove_dir(path){
                                Ok(_)=>{},
                                Err(_)=>std::process::exit(-60)
                            };
                           }
                          },
        Err(_)=>std::process::exit(-60)
    } 

        }
}

fn main() {
    

    let args: Vec<String> = env::args().collect();
    

    if args.len() < 2 {
         println!("Invalid command");
         std::process::exit(255);
     
     } 
      
     let a = args[1].as_str();
 
     match a{
         "pwd"=>pwd(),
         "echo"=>{ 
            if args.len()<=2{
                std::process::exit(255);
            }else{
                echo(args);
            }
         },
         "cat"=>cat(args),
         "mkdir"=>mkdir(args),
         "mv"=>mv(args),
         "ln"=>ln(args),
         "rmdir"=>rmdir(args),

         _=>{
            println!("Invalid command");
            std::process::exit(255);
        }
        

         
     }


}
