/*
    src/modules/model/png.rs
    aEon@khaa.pk
 */

/* 
    https://www.w3.org/TR/PNG-Structure.html
 */

use std::io::Write;
/* 
    When    
 */
//use std::process::Command;

//use byteorder::{ByteOrder, LittleEndian, BigEndian};
use flate2::{write::DeflateEncoder}; 
use flate2::Compression;
use flate2::write::ZlibEncoder;
use crate::modules::model::content::{Content, ContentBody};

//use crate::modules::khaa_pk_read_write;
//use khaa_pk_read_write::read_write::write;

/* Native function call */
extern {

    fn big_endian_read_u32(ptr: *const u8) -> u32;
    fn big_endian_write_u32(ptr: *mut u8, value: u32);
    fn draw_circle(pixels: *mut u8, height: u32, width: u32, a: u32, b: u32, r: u32, color: u8);
    fn draw_line(pixels: *mut u8, height: u32, width: u32, x1: u32, y1: u32, x2: u32, y2: u32,  color: u8);
    fn update_crc(crc: u32, buf: *const u8, len: u32) -> u32;
    fn write_png(name: *const u8, ptr: *const u8, len: u32);
}

/*
#[derive(Clone)]
pub struct ChunkIdat {
    
    pub length: Vec<u8>,
    pub type_name: Vec<u8>,
    pub data: Vec<u8>,
    pub crc: Vec<u8>
}

impl ChunkIdat {

    pub fn new(&self) -> Self {

        let type_name = self.type_name.to_vec();
        let length = self.length.to_vec();
        let data = self.data.to_vec();
        let crc = self.crc.to_vec();
        
        Self {

            type_name,
            length,
            data,
            crc
        }
    }
}
 */

#[derive(Clone)]
pub struct Chunk {
    
    pub length: Vec<u8>,
    pub type_name: Vec<u8>,
    pub data: Vec<u8>,
    pub crc: Vec<u8>
}

impl Chunk {

    pub fn new(&self) -> Self {

        let length = self.length.to_vec();
        let type_name = self.type_name.to_vec();        
        let data = self.data.to_vec();
        let crc = self.crc.to_vec();
        
        Self {

            length,
            type_name,            
            data,
            crc
        }
    }
}

#[derive(Clone)]
pub struct Png {
    
    pub height: usize,
    pub width: usize,
    pub background: String,
    pub foreground: String,
    
    pub signature: Vec<u8>,
    pub ihdr: Chunk,    
    pub plte: Chunk,
    pub idat: Chunk,
    pub iend: Chunk,
}

impl Png {

    pub fn new(&self) -> Self {
                
        let height = self.height;
        let width = self.width;
        let background = self.background.to_string();
        let foreground = self.foreground.to_string();

        // The PNG file
        let signature = self.signature.clone();
        let ihdr = self.ihdr.clone();
        let plte = self.plte.clone();
        let idat = self.idat.clone();
        let iend = self.iend.clone();

        Self {            
            //signature: vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],            
            height,
            width,
            background,
            foreground,
            signature,
            ihdr,
            plte,
            idat,
            iend,            
        }
    }


    pub fn generate_ihdr(&mut self) {

        /*
         //   println!("Type: {}{}{}{}", self.ihdr.type_name[0] as char, self.ihdr.type_name[1] as char, self.ihdr.type_name[2] as char, self.ihdr.type_name[3] as char);
         */
        
        // -------------------
        //  SET CHUNK LENGTH
        // -------------------
        /* This is working.... both lines*/
        /* BigEndian::write_u32(&mut self.idat.length, self.idat.data.len().try_into().unwrap()); */               
        unsafe { big_endian_write_u32(self.ihdr.length.as_mut_ptr(), self.ihdr.data.len().try_into().unwrap()) }; 
        
        unsafe { big_endian_write_u32((&mut self.ihdr.data[0..4]).as_mut_ptr(), self.width.try_into().unwrap()) }; 
        unsafe { big_endian_write_u32((&mut self.ihdr.data[4..8]).as_mut_ptr(), self.height.try_into().unwrap()) };
        /*
        for i in 0 .. self.ihdr.data.len() {

            println!("{} -> {}", i, self.ihdr.data[i]); 
        }
         */

        // -----------------
        //   SET CHUNK CRC
        // -----------------                                
        let input = [&self.ihdr.type_name[..], &self.ihdr.data[..]].concat();
        let mut crc = unsafe { update_crc(0xFFFFFFFFu32, input.as_ptr(), input.len().try_into().unwrap()) };
        crc = crc ^ 0xFFFFFFFFu32;                                
        unsafe { big_endian_write_u32(self.ihdr.crc.as_mut_ptr(), crc) };

        /*    
         //   println!("CRC: {}(0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x})", crc, self.ihdr.crc[0], self.ihdr.crc[1], self.ihdr.crc[2], self.ihdr.crc[3]);
         //   println!("Length(base10): {}", unsafe {big_endian_read_u32(self.ihdr.length.as_ptr())});
         */
    } 
/*} */

    pub fn generate_plte(&mut self) {
        
        /*
         //   println!("Type: {}{}{}{}", self.plte.type_name[0] as char, self.plte.type_name[1] as char, self.plte.type_name[2] as char, self.plte.type_name[3] as char);
         */
        
        // -------------------
        //  SET CHUNK LENGTH
        // -------------------
        /* This is working.... both lines*/
        /* BigEndian::write_u32(&mut self.idat.length, self.idat.data.len().try_into().unwrap()); */               
        unsafe { big_endian_write_u32(self.plte.length.as_mut_ptr(), self.plte.data.len().try_into().unwrap()) }; 

        // -----------------
        //   SET CHUNK CRC
        // -----------------                                
        let mut input = [&self.plte.type_name[..], &self.plte.data[..]].concat();
        let mut crc = unsafe { update_crc(0xFFFFFFFFu32, input.as_mut_ptr(), input.len().try_into().unwrap()) };
        crc = crc ^ 0xFFFFFFFFu32;                                
        unsafe { big_endian_write_u32(self.plte.crc.as_mut_ptr(), crc) };

        /*
         //   println!("CRC: {}(0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x})", crc, self.plte.crc[0], self.plte.crc[1], self.plte.crc[2], self.plte.crc[3]);
         //   println!("Length(base10): {}", unsafe {big_endian_read_u32(self.plte.length.as_ptr())});
         */
    }

    pub fn generate_idat(&mut self) {
        
        /*
            //println!("Type: {}{}{}{}", self.idat.type_name[0] as char, self.idat.type_name[1] as char, self.idat.type_name[2] as char, self.idat.type_name[3] as char);
         */
         
        // Draw line here...                
        unsafe { draw_line(self.idat.data.as_mut_ptr(), self.height.try_into().unwrap(), self.width.try_into().unwrap(), 205, 200, 2000, 700, 1) };
        // Draw circle here...
        unsafe { draw_circle(self.idat.data.as_mut_ptr(), self.height.try_into().unwrap(), self.width.try_into().unwrap(), 800, 400, 50, 3) };

         
        /*
            The data is already initialized to 0 at each index, this is your background color index in PLTE chunk
         */
        /*
        for i in 0 .. self.idat.data.len() {
           
                self.idat.data[i] = 0;
           
        }
         */

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&self.idat.data).unwrap();

        let result = encoder.finish();

        match result {

            Ok(data) => {

                // ------------------
                //   SET CHUNK DATA
                // ------------------
                /*for i in 0 .. data.len() {
                    
                    print!("{:2x} ", data[i]);
                }*/
                /*self.idat.data.resize(data.len(), 0);        
                for i in 0 .. data.len() {

                    self.idat.data[i] = data[i];
                }*/

                self.idat.data = data;
            }

            Err(error) => {

                panic!("Error while compressing data: {:?}", error)
            }
        }

        // ------------------
        //   SET CHUNK DATA
        // ------------------
        /*
        self.idat.data.resize(data.len(), 0);        
        for i in 0 .. data.len() {

            self.idat.data[i] = data[i];
        }

        self.idat.data = data;
        */

        // -------------------
        //  SET CHUNK LENGTH
        // -------------------
        /* This is working.... both lines*/
        /* BigEndian::write_u32(&mut self.idat.length, self.idat.data.len().try_into().unwrap()); */               
        unsafe { big_endian_write_u32(self.idat.length.as_mut_ptr(), self.idat.data.len().try_into().unwrap()) };

        // -----------------
        //   SET CHUNK CRC
        // -----------------
        //self.png_crc(self.idat.data, self.idat.type_name/*, self.idat.crc.as_mut()*/);
        let input = [&self.idat.type_name[..], &self.idat.data[..]].concat();
        let mut crc = unsafe { update_crc(0xFFFFFFFFu32, input.as_ptr(), input.len().try_into().unwrap()) };
        crc = crc ^ 0xFFFFFFFFu32;                                
        unsafe { big_endian_write_u32(self.idat.crc.as_mut_ptr(), crc) };
        //println!("{}, {}, {}, {} crc = {}", self.idat.crc[3], self.idat.crc[2], self.idat.crc[1], self.idat.crc[0], crc);

        /*
         //   println!("CRC: {}(0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x})", crc, self.idat.crc[0], self.idat.crc[1], self.idat.crc[2], self.idat.crc[3]);
         //   println!("Length(base10): {}", unsafe {big_endian_read_u32(self.idat.length.as_ptr())});
         */

    }

    pub fn generate_idat_old(&mut self) {

        println!("Type: {}{}{}{}", self.idat.type_name[0] as char, self.idat.type_name[1] as char, self.idat.type_name[2] as char, self.idat.type_name[3] as char);        
       
        for i in 0 .. self.idat.data.len() {

            self.idat.data[i] = 1;
        }
               
        // Deflate the pixel data using flate2
       let /*mut*/ de_flater = DeflateEncoder::new(&mut self.idat.data, Compression::default());
       //////////////////////////////////////////////////////////////////
       /*
       let result = de_flater.finish();
        */
       ////////////////////////////////////////////////////////////////      
       de_flater.finish().unwrap();

       // Create a new ZlibEncoder with a compression level of 6
       let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default()); 

       // Write the deflated pixel data to the ZlibEncoder
       encoder.write_all(&mut self.idat.data).unwrap();

       // Close the encoder to finish the compression process
       let result = encoder.finish();
       
       // The compressed data with zlib wrapper is now in the result.unwrap()

       match result {
            Ok(data) => {

                // ------------------
                //   SET CHUNK DATA
                // ------------------
                self.idat.data.resize(data.len(), 0);
                //self.idat.data = data;

                for i in 0 .. data.len() {

                    self.idat.data[i] = data[i];
                }
                                
                // -------------------
                //  SET CHUNK LENGTH
                // -------------------
                /* This is working.... both lines*/
                /* BigEndian::write_u32(&mut self.idat.length, self.idat.data.len().try_into().unwrap()); */               
                unsafe { big_endian_write_u32(self.idat.length.as_mut_ptr(), self.idat.data.len().try_into().unwrap()) };
            }

            Err(error) => {

                panic!("Error while compressing data: {:?}", error)
            }
       }

        // -------------------
        //  SET CHUNK LENGTH
        // -------------------
        /* This is working.... both lines*/
        /* BigEndian::write_u32(&mut self.idat.length, self.idat.data.len().try_into().unwrap()); */               
        //unsafe { big_endian_write_u32(self.idat.length.as_mut_ptr(), self.idat.data.len().try_into().unwrap()) };    

        // -----------------
        //   SET CHUNK CRC
        // -----------------
        //self.png_crc(self.idat.data, self.idat.type_name/*, self.idat.crc.as_mut()*/);
        let input = [&self.idat.type_name[..], &self.idat.data[..]].concat();
        let mut crc = unsafe { update_crc(0xFFFFFFFFu32, input.as_ptr(), input.len().try_into().unwrap()) };
        crc = crc ^ 0xFFFFFFFFu32;                                
        unsafe { big_endian_write_u32(self.idat.crc.as_mut_ptr(), crc) };
        //println!("{}, {}, {}, {} crc = {}", self.idat.crc[3], self.idat.crc[2], self.idat.crc[1], self.idat.crc[0], crc);

        println!("CRC: {}(0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x})", crc, self.idat.crc[0], self.idat.crc[1], self.idat.crc[2], self.idat.crc[3]);
        println!("Length(base10): {}", unsafe {big_endian_read_u32(self.idat.length.as_ptr())});

    }
    
    pub fn generate_idat_older(&mut self) {

        println!("Type: {}{}{}{}", self.idat.type_name[0] as char, self.idat.type_name[1] as char, self.idat.type_name[2] as char, self.idat.type_name[3] as char);        
       
       // self.idat.data has the uncompressed pixel data. 1 compress it to deflate and then wrap it with zlib

       for i in 0 .. self.idat.data.len() {

            self.idat.data[i] = 1;
       }
              
       // Deflate the pixel data using flate2
       let /*mut*/ de_flater = DeflateEncoder::new(&mut self.idat.data, Compression::default());
       //////////////////////////////////////////////////////////////////
       /*
       let result = de_flater.finish();
        */
       ////////////////////////////////////////////////////////////////      
       de_flater.finish().unwrap();

       // Create a new ZlibEncoder with a compression level of 6
       let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default()); 

       // Write the deflated pixel data to the ZlibEncoder
       encoder.write_all(&mut self.idat.data).unwrap();

       // Close the encoder to finish the compression process
       let result = encoder.finish();
       
       // The compressed data with zlib wrapper is now in the result.unwrap()

       match result {
            Ok(data) => {

                // ------------------
                //   SET CHUNK DATA
                // ------------------
                self.idat.data.resize(data.len(), 0);
                self.idat.data = data;
                                
                // -------------------
                //  SET CHUNK LENGTH
                // -------------------
                /* This is working.... both lines*/
                /* BigEndian::write_u32(&mut self.idat.length, self.idat.data.len().try_into().unwrap()); */               
                unsafe { big_endian_write_u32(self.idat.length.as_mut_ptr(), self.idat.data.len().try_into().unwrap()) };
            }

            Err(error) => {

                panic!("Error while compressing data: {:?}", error)
            }
       }

       /*
       match result {

            Ok(data) => {

                //println!("---------> 1 {}", data.len());

                // Create a new ZlibEncoder with a compression level of 6
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default()); 
                // Write the deflated pixel data to the ZlibEncoder
                //let result = encoder.write_all( &mut self.idat.data);
                let result = encoder.write_all(data);

                /////////////////////////////////////////////////////////////////////////////////////                
                match result {

                    Ok(_) => {

                        let result = encoder.finish();

                        match result {

                            Ok(data) => {
                                
                                // ------------------
                                //   SET CHUNK DATA
                                // ------------------
                                self.idat.data = data.clone();
                                
                                // -------------------
                                //  SET CHUNK LENGTH
                                // -------------------
                                /* This is working.... both lines*/
                                /* BigEndian::write_u32(&mut self.idat.length, self.idat.data.len().try_into().unwrap()); */               
                                unsafe { big_endian_write_u32(self.idat.length.as_mut_ptr(), self.idat.data.len().try_into().unwrap()) }; 

                                /*
                                // -------------------
                                //   SET CHUNK Type
                                // -------------------
                                self.idat.type_name[0] = 0x49;
                                self.idat.type_name[1] = 0x44;
                                self.idat.type_name[2] = 0x41;
                                self.idat.type_name[0] = 0x54;
                                 */

                                // -----------------
                                //   SET CHUNK CRC
                                // -----------------
                                //self.png_crc(self.idat.data, self.idat.type_name/*, self.idat.crc.as_mut()*/);
                                let input = [&self.idat.type_name[..], &self.idat.data[..]].concat();
                                let mut crc = unsafe { update_crc(0xFFFFFFFFu32, input.as_ptr(), input.len().try_into().unwrap()) };
                                crc = crc ^ 0xFFFFFFFFu32;                                
                                unsafe { big_endian_write_u32(self.idat.crc.as_mut_ptr(), crc) };
                                //println!("{}, {}, {}, {} crc = {}", self.idat.crc[3], self.idat.crc[2], self.idat.crc[1], self.idat.crc[0], crc);

                                println!("CRC: {}(0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x})", crc, self.idat.crc[0], self.idat.crc[1], self.idat.crc[2], self.idat.crc[3]);
                                println!("Length(base10): {}", unsafe {big_endian_read_u32(self.idat.length.as_ptr())});
                            }

                            Err(error) => {

                                panic!("Error while compressing data: {:?}", error)
                            }
                        }
                    }

                    Err(error) => {
                        
                        panic!("Error while compressing data: {:?}", error)
                    }
                }                
                /////////////////////////////////////////////////////////////////////////////////////////////////
            }

            Err(error) => {

                panic!("Error while compressing data: {:?}", error)
            }
       }
        */       
    }

    pub fn generate_iend(&mut self) {
        
        /*
         //   println!("Type: {}{}{}{}", self.iend.type_name[0] as char, self.iend.type_name[1] as char, self.iend.type_name[2] as char, self.iend.type_name[3] as char);
         */

        // -----------------
        //   SET CHUNK CRC
        // -----------------
        let mut crc = unsafe { update_crc(0xFFFFFFFFu32, self.iend.type_name.as_ptr(), self.iend.type_name.len().try_into().unwrap()) };
        crc = crc ^ 0xFFFFFFFFu32;                                
        unsafe { big_endian_write_u32(self.iend.crc.as_mut_ptr(), crc) };

        /*
         //   println!("CRC: {}(0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x})", crc, self.iend.crc[0], self.iend.crc[1], self.iend.crc[2], self.iend.crc[3]);
         //   println!("Length(base10): {}", unsafe {big_endian_read_u32(self.iend.length.as_ptr())});
         */
    }

    pub fn generate_image(&self, name: String) {

        // //////////////////////
        //      The image      // 
        // //////////////////////       
        // Signature
        let mut content = Content {content: self.signature.clone(), content_length: self.signature.len()};
        // IHDR
        content.set_content_vec(self.ihdr.length.clone());
        content.set_content_vec(self.ihdr.type_name.clone());
        content.set_content_vec(self.ihdr.data.clone());
        content.set_content_vec(self.ihdr.crc.clone());
        // PLTE
        content.set_content_vec(self.plte.length.clone());
        content.set_content_vec(self.plte.type_name.clone());

        /*
        for i in 0 .. self.idat.data.clone().len() {

            print!("{:2x} ", self.idat.data.clone()[i]);
        }
         */
        content.set_content_vec(self.plte.data.clone());
        content.set_content_vec(self.plte.crc.clone());
        // IDAT
        content.set_content_vec(self.idat.length.clone());
        content.set_content_vec(self.idat.type_name.clone());
        content.set_content_vec(self.idat.data.clone());
        content.set_content_vec(self.idat.crc.clone());
        // IEND
        content.set_content_vec(self.iend.length.clone());
        content.set_content_vec(self.iend.type_name.clone());
        //content.set_content_vec(self.iend.data.clone());
        content.set_content_vec(self.iend.crc.clone());
        
        /*
        for i in 0 .. /*content.get_content().as_bytes().len()*/ content.get_content_length() {

            println!("{} -->  {:02x}", i, content.get_content().as_bytes()[i]);
        }
         */
        
        //write(&name, content); 

       unsafe { write_png(name.as_ptr(), content.get_content().as_ptr(), content.get_content_length().try_into().unwrap()) };
       

       /*
            To call an external executable code from this Rust program
        */
       /*
       let output = Command::new("D:\\Khaa.pk\\JUGAR\\main.exe").arg("arg1 arg2 arg3 arg").output().expect("failed to execute process"); 

       println!("status: {}", output.status);
       println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
       println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        */

    }

}

/*
pub trait PngBody {
    
    fn new(&mut self, height: usize, width: usize, background: String, foreground: String) -> Self;
}
 */

/*
impl PngBody for Png {

    fn new (&mut self, h: usize, w: usize, b: String, f: String) -> Self {
                
        self.height = h;
        self.width = w;
        self.background = b;
        self.foreground = f;

        //let type_name: Vec<u8> = vec![0x49, 0x44, 0x41, 0x54];
        self.idat = ChunkIdat {type_name: vec![0x49, 0x44, 0x41, 0x54], length: vec![0x00, 0x00, 0x00, 0x00], data: vec![0x00, 0x00, 0x00, 0x00], crc: vec![0x00, 0x00, 0x00, 0x00]};

        // return         
        //Png {height: self.height, width: self.width, background: self.background.to_string(), foreground: self.foreground.to_string(), idat: Chunk_IDAT {type_name: vec![0x49, 0x44, 0x41, 0x54], length: vec![0x00, 0x00, 0x00, 0x00], data: vec![0x00, 0x00, 0x00, 0x00], crc: vec![0x00, 0x00, 0x00, 0x00]}}

        self.clone()        
    }   
}
 */