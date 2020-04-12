use cmd_lib::*;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::{env, process::Command, thread, time};
pub mod pdf;
use pdf::pdfmaker;

fn main() {
     let args: Vec<String> = env::args().collect();
     if args.len() < 3 {
         
         pdfmaker(args[1].parse::<u8>().unwrap());
         println!("Prepared, Your Assignment is Done");
         return ()
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


fn create_output(command:String,path:String){

        let mut rng = rand::thread_rng();

        let  display_number:u8=rng.gen();

        let xvfb_command=format!("Xvfb :{} -screen 5 100x100x8 &",&display_number);
   
        Command::new("sh")
            .arg("-c")
            .arg(&xvfb_command)
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

        thread::sleep(time::Duration::from_millis(1000));


        let xterm_process=Command::new("sh")
            .arg("-c")
            .arg(format!("ps -aux |grep '{}'",format!("xterm -hold -fa monaco -fs 25 -bg black -fg green -maximized -e {}",command))) 
            .output()
            .expect("failed to execute process");

        let xterm_process:Vec<&str>=str::from_utf8(&xterm_process.stdout).unwrap().split(" ").collect(); 
        let xterm_process_pid = xterm_process[7];

        Command::new("sh")
            .arg("-c")
            .arg(format!("kill {}",xterm_process_pid)) 
            .output()
            .expect("failed to execute process");
                    
        let xvfb_process=Command::new("sh")
                            .arg("-c")
                            .arg(format!("ps -aux |grep 'Xvfb :{}'",display_number)) 
                            .output()
                            .expect("failed to execute process");

        let xvfb_process:Vec<&str>=str::from_utf8(&xvfb_process.stdout).unwrap().split(" ").collect(); 
        let xvfb_process_pid = xvfb_process[7];

        Command::new("sh")
            .arg("-c")
            .arg(format!("kill {}",xvfb_process_pid)) 
            .output()
            .expect("failed to execute process");
        println!("{}", display_number);


}


