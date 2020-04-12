use cmd_lib::*;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::{env, process::Command, thread, time};
pub mod pdf;
pub mod java;
use pdf::pdfmaker;
use java::{java,create_output};


fn main() {
     let mut args: Vec<String> = env::args().collect();
     if args.len() < 3 {
         
         pdfmaker(args[1].parse::<u8>().unwrap());
         println!("Prepared, Your Assignment is Done");
         return ()
     }

     else if args[1] == "java"{
        args[1] = String::from("");
        return java(args)
    }
    let contents = fs::read_to_string("ob.txt").expect("Something went wrong reading the file");
    let mut vec: Vec<String> = Vec::new();
    vec.push(String::from(""));
    let mut i = 0;
    let mut skip = false;
    for line in contents.lines() {
        if line == "---!---" {
            i = i + 1;

            vec.push(String::from(String::from("echo \"") + &args.join(" ")) + "\"");

            skip = true;
            continue;
        } else if skip {
            skip = false;
            continue;
        }
        vec[i].push_str("\n");
        vec[i].push_str(line);
    }
    let mut files = String::from("test/ob");
    for x in 1..i+1 {
        files.push_str(&x.to_string());
        files.push_str(".sh");
        fs::write(&files, &vec[x]).expect("Unable to write file");
        files = String::from("test/ob");
    }
    
    thread::sleep(time::Duration::from_millis(2000));
    let mut i = vec.len();
    println!("{}",i);
    for t in 1..i{
        files.push_str(&t.to_string());
        files.push_str(".sh");
        let command = String::from("./") + &files;
        let v = run_fun!("cat ls.txt | bash {}",&command).expect("helo");
        println!("{}",v);
        fs::write(format!("test/bo{}.txt",t),&v).expect("hlfs");
        create_output(String::from(format!("cat test/bo{}.txt",t)),String::from(format!("test/work{}.png",t.to_string()))); 
        files = String::from("test/ob");    
         
    }


    
    //create_output(String::from("cat ls.txt |./ob1.sh"),String::from("work.png"))
   
}

