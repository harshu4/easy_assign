use std::fs;
use cmd_lib::*;
use std::fs::File;
use std::{env, process::Command, thread, time};
use rand::Rng;

pub fn java(veci:Vec<String>) {
   let inputs = fs::read_to_string("test/input.txt").expect("Something went wrong reading the file");
   let mut ins:Vec<String> = Vec::new();
   let mut v = 0;
   for line in inputs.lines(){
       if line.contains("#786"){
        line.replace("#786",&veci.join(" "));
       }
       else if line.trim() == "\n" || line.trim() == "" || line.trim() ==  " "{
        continue;
    }
    
       else if line.contains("---!---"){
           v +=1;
           ins.push(String::from(""));
           continue;
       }
       ins[v-1].push_str(line);
       ins[v-1].push_str("\n");
    }
    let mut ans = String::from("test/ans");
    for y in 0..v{
        ans.push_str(&y.to_string());
        ans.push_str(".txt");
        fs::write(&ans, &ins[y]).expect("Unable to write file");
        ans = String::from("test/ans");
    }
    

   let contents = fs::read_to_string("test/newfile.txt").expect("Something went wrong reading the file");
   let mut vec: Vec<String> = Vec::new();
   let mut cot:String = String::new();
   vec.push(String::from(""));
   let mut i = 0;
   for line in contents.lines() {
       cot = line.replace("\u{00a0}","");
       if line == "---!---" {
           i += 1;
           vec.push(String::from(""));
           continue;
       }
       else if line.contains("Vaijul") || line.contains("Balar") || line.contains("1804101070007"){
          cot =  line.replace("Vaijul",&veci[2]);
          cot = cot.replace("180410107007",&veci[4]);
          cot = cot.replace("Balar",&veci[3]);
          vec[i-1].push_str("\n");
          vec[i-1].push_str(&cot);
          continue;
       }
       
       vec[i-1].push_str("\n");
       vec[i-1].push_str(&cot);
   }
   let mut files = String::from("test/ob");
   for x in 0..i {
       files.push_str(&x.to_string());
       files.push_str(".java");
       fs::write(&files, &vec[x]).expect("Unable to write file");
       files = String::from("test/ob");
   }
   
   thread::sleep(time::Duration::from_millis(2000));
   let mut i = vec.len();
   println!("Hello this is i {}",i);
   let mut fileso = String::from("test/ob");
   for t in 0..i-1{
       files.push_str(&t.to_string());
       files.push_str(".java");

       if (t >= 16 && t <= 19){
           
           let vi = run_fun!("javac --module-path /usr/share/openjfx/lib --add-modules=javafx.controls,javafx.fxml,javafx.base,javafx.media,javafx.web,javafx.swing {}",&files).expect("helo");
           fileso = String::from("ob");
           fileso.push_str(&t.to_string());
           create_output(String::from(format!("java --module-path /usr/share/openjfx/lib --add-modules=javafx.controls,javafx.fxml,javafx.base,javafx.media,javafx.web,javafx.swing {}",&fileso)),String::from(format!("test/work{}.png",t.to_string()))); 
           fileso = String::from("ob");  
           files = String::from("test/ob")  ;
            continue;
           
        }
        if (t  == 24){
            let v = run_fun!("cat test/ans{}.txt | java {} 123.txt",t,&files).expect("helo");
       fs::write(format!("test/bo{}.txt",t),&v).expect("hlfs");
       create_output(String::from(format!("cat test/bo{}.txt",t)),String::from(format!("test/work{}.png",t.to_string()))); 
       files = String::from("test/ob");   
       continue;
           
        }
       let v = run_fun!("cat test/ans{}.txt | java {}",t,&files).expect("helo");
       fs::write(format!("test/bo{}.txt",t),&v).expect("hlfs");
       create_output(String::from(format!("cat test/bo{}.txt",t)),String::from(format!("test/work{}.png",t.to_string()))); 
       files = String::from("test/ob");   
        
   }


   
   //create_output(String::from("cat ls.txt |./ob1.sh"),String::from("work.png"))
  
}



pub fn create_output(command: String, path: String){
    let mut rng = rand::thread_rng();
    println!{"call received"}
    let display_number: u16 = rng.gen();

    let xvfb_command=format!("Xvfb :{} -screen 0 1280x720x24 -ac +extension GLX +render -noreset &",&display_number);

        Command::new("sh")
            .arg("-c")
            .arg(&xvfb_command)
            .spawn()
            .expect("failed to spawn");


        thread::sleep(time::Duration::from_millis(1000));

        Command::new("sh")
                    .env("DISPLAY", format!(":{}",&display_number))
                    .arg("-c")
                    .arg(format!("xterm -hold -fa monaco -fs 25 -bg black -fg green -maximized -e '{}' &",command))
                    .spawn()
                    .expect("failed to spawn");


    thread::sleep(time::Duration::from_millis(1000));

    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "xwd -display :{} -root -silent | convert xwd:- png:{}",
            &display_number, path
        ))
        .spawn()
        .expect("failed to spawn");

}




pub fn create_output_image(command: String, path: String){
    let mut rng = rand::thread_rng();
    println!{"call received"}
    let display_number: u16 = rng.gen();

    Command::new("sh")
        .arg("-c")
        .arg(format!("Xvfb :{} -screen 5 100x100x8 &", &display_number))
        .spawn()
        .expect("failed to spawn");

    thread::sleep(time::Duration::from_millis(1000));

    Command::new("sh")
     
        .arg("-c")
        .arg(format!(
            "cd test && xterm -hold -e {} ",
            command
        ))
        .spawn()
        .expect("failed to spawn");

    thread::sleep(time::Duration::from_millis(4000));

    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "xwd -display :{} -root -silent | convert xwd:- png:{}",
            0, path
        ))
        .spawn()
        .expect("failed to spawn");

}