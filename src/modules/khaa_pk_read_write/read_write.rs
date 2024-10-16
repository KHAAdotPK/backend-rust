/*
    src/modules/khaa_pk_read_write/read_write.rs
    aEon@khaa.pk
 */

 use std::{fs::{self, File}, io::{Read, Write}, path::Path};
 use crate::modules::model::content::{Content, ContentBody};
 
 pub fn exists(name: &str) -> bool {

    return Path::new(name).exists();  
 }

 // pub fn read(&'static str)-> Content {
 pub fn read(name: &str) -> Content {
 
     let mut buffer: Vec<u8>;
     let path = Path::new(name);
 
     let file = File::open(&path);

     //println!("File to open = {}", name);
 
     match file {
 
         Err(_why) => {
 
             panic!("couldn't open {}: {}", path.display().to_string(), _why);
 
             //return Content{content: Vec::new(), content_length: 0};
         }
 
         Ok(mut f) => {
 
             let metadata = fs::metadata(name).unwrap();
 
             buffer = vec![0; metadata.len() as usize];
 
             f.read(&mut buffer).unwrap();
 
             return Content {content: buffer, content_length: metadata.len() as usize};
         }
     }
         
 }

 pub fn write(name: &str, content: Content) -> Content {

     let path = Path::new(name);

     let file = File::create(&path);

     match file {

        Err(_why) => {
 
            panic!("couldn't open {}: {}", path.display().to_string(), _why);

            //return Content{content: Vec::new(), content_length: 0};
        }

        Ok(mut f) => {

            //let metadata = fs::metadata(name).unwrap();
 
             //buffer = vec![0; metadata.len() as usize];
 
            //f.read(&mut buffer).unwrap();
            f.write_all(&content.get_content_vec()).unwrap();
        }

     }

     Content {content: Vec::new(), content_length: 0}
 }