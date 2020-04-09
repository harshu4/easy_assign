extern crate rand;
use std::{thread, time, env, process::Command};
use rand::Rng;

fn main(){
    create_output(String::from("pwd"),String::from("/home/ddl/work.png"))
}

fn create_output(command:String,path:String){

        let mut rng = rand::thread_rng();

        let  display_number:u8=rng.gen();
   
        Command::new("sh")
            .arg("-c")
            .arg(format!("Xvfb :{} -screen 5 100x100x8 &",&display_number))
            .spawn()
            .expect("failed to spawn");
            

        thread::sleep(time::Duration::from_millis(1000));
            
        Command::new("sh")
                    .env("DISPLAY", format!(":{}",&display_number))
                    .arg("-c")
                    .arg(format!("xterm -hold -fa monaco -fs 25 -bg black -fg green -maximized -e {} &",command))
                    .spawn()
                    .expect("failed to spawn");
           
        thread::sleep(time::Duration::from_millis(1000));

        Command::new("sh")
                    .arg("-c")
                    .arg(format!("xwd -display :{} -root -silent | convert xwd:- png:{}",&display_number,path))
                    .spawn()
                    .expect("failed to spawn");     
}

