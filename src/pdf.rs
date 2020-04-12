use printpdf::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

pub fn pdfmaker(size:u8){

    let mypdf = PdfDocument::new("OS Assignment", Mm(150.0), Mm(250.0), "Practical 0");
    let font = (mypdf.0).add_external_font(File::open("mono.ttf").unwrap()).unwrap();
    let mut file2 = File::open("test/question.txt").expect("question not found");
    let mut contents2 = String::from("");
    file2.read_to_string(&mut contents2).expect("lol");
    let mut vec:Vec<&str> = Vec::new();
    

    for line in contents2.split("\n"){
        vec.push(line);
    }
    let mut vec_index = 0;
    for x in 0..size-1{
        let mut current_layer;
        if x==0{
            current_layer = (mypdf.0).get_page(mypdf.1).get_layer(mypdf.2);
        }else{
            let (page, layer) = (mypdf.0).add_page(Mm(150.0), Mm(250.0),format!("Practical {}",x+1));
            current_layer = (mypdf.0).get_page(page).get_layer(layer);
        }
        
        let mut file = File::open(format!("test/ob{}.sh",x+1)).expect("lol");
        
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("lol");
        let mut y=240.0;
        for x in vec_index..vec.len(){
            if vec[x].contains("---!---"){
                vec_index +=1;
                break;
            }
            vec_index +=1;
            current_layer.use_text(String::from(vec[x]), 12, Mm(5.0), Mm(y), &font);
            y=y-5.0;
            
        }



        for line in contents.split("\n"){
            if y < 10.0{
                y=240.0;
                let (page, layer) = (mypdf.0).add_page(Mm(150.0), Mm(250.0),format!("Practicle {}",x));
                current_layer = (mypdf.0).get_page(page).get_layer(layer);
            }
            current_layer.use_text(line, 12, Mm(5.0), Mm(y), &font);
            y=y-5.0;
            
        }
        if y-100.0 < 5.0 {
         	y=240.0;
                let (page, layer) = (mypdf.0).add_page(Mm(150.0), Mm(250.0),format!("Practicle {}",x));
                current_layer = (mypdf.0).get_page(page).get_layer(layer);
        }
        
        let mut image_file = File::open(format!("test/work{}.png",x+1)).unwrap();
        let image = Image::try_from(image::png::PngDecoder::new(&mut image_file).unwrap()).unwrap();
       image.add_to_layer(current_layer.clone(),Some(Mm(5.0)), Some(Mm(y-100.0)), None, None, None, None);
       
    }
    (mypdf.0).save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
    //println!("{}", contents);
    //let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
    //let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0),"Page 2, Layer 1");
    //doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
}
