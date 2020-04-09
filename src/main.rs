extern crate rand;

use std::{thread, time, env, process::Command};
use rand::Rng;

fn main(){
    generate_output(String::from("/home/ddl/wow.png"),String::from("ls"));
  
}


fn generate_output(path:String,command:String){

    let mut rng = rand::thread_rng();

    let  display_number:u8=rng.gen();

  
   
       
    Command::new("sh")
                .arg("-c")
                .arg(format!("Xvfb :{} -screen 5 100x100x8 &",&display_number))
                .spawn()
                .expect("failed to execute process");

    let mut i=0;
    loop{
        if(i==100){
            break;
        }
        i=i+100;
    }

    Command::new("sh")
                .env("DISPLAY", format!(":{}",&display_number))
                .arg("-c")
                .arg(format!("xterm -hold -maximized -e {} &",command))
                .spawn()
                .expect("failed to execute process");

 

    Command::new("sh")
                .arg("-c")
                .arg(format!("xwd -display :{} -root -silent | convert xwd:- png:{}",&display_number,path))
                .spawn()
                .expect("failed to execute process");
}