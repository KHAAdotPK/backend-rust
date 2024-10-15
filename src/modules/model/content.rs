/*
    src/modules/model/content.rs
    aEon@khaa.pk
 */

 use std::{str};

 // Properties
 pub trait ContentBody {
     
     fn get_content(&self) -> &str;

     //fn get_content(&self) -> String;
     fn get_content_length(&self) -> usize;
     fn get_content_vec(&self) -> Vec<u8>;

     fn set_content(&mut self, content: &str) -> usize;
     fn set_content_vec(&mut self, content: Vec<u8>) -> usize;
 }
 
 // Methods
 pub struct Content {
 
     pub content: Vec<u8>,
     pub content_length: usize,
 }
 
 impl ContentBody for Content {
             
     fn get_content(&self) -> &str {
                
         //https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
         return unsafe { str::from_utf8_unchecked(&self.content) };         
     }

     fn get_content_vec(&self) -> Vec<u8> {

        return self.content.clone();
     }
      
     /* 
     fn get_content(&self) -> String {
         
        self.content
     } 
      */
      
     fn get_content_length(&self) -> usize {
         
         return self.content_length;
     }
     
     fn set_content(&mut self, content: &str) -> usize {
                
        if self.get_content_length() > 0 {

            //self.content = self.content + content.as_bytes().to_vec();

            let size = self.get_content_length();
            
            self.content.resize(content.len() + size, 0);

            for i in 0 .. content.len() {

                self.content[size + i] = content.as_bytes().to_vec()[i];
            }
        }
        else {

            self.content = content.as_bytes().to_vec();
        }

        self.content_length = self.content_length + content.len();

        content.len()
     }

     fn set_content_vec(&mut self, content: Vec<u8>) -> usize {

        if self.get_content_length() > 0 {

            let size = self.get_content_length();

            self.content.resize(content.len() + size, 0);
            
            for i in 0 .. content.len() {
               
                self.content[size + i] = content[i];
            }   
        }
        else {

            self.content = content.clone();
        }

        self.content_length = self.content_length + content.len();
        
        content.len()
     }
 }