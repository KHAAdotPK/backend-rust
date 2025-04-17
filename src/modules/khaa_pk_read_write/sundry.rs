/*
    src/modules/khaa_pk_read_write/dict.rs
    aEon@khaa.pk
 */

use std::{io::{Read, Write}, net::TcpStream, ptr::null, str};
use crate::modules::{model::{content::{Content, ContentBody}, dict::{Dict, DictBody}}, constants::{self, DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT, DEFAULT_PNG_BIT_DEPTH}};
use regex::bytes::Regex; // For byte string matching

use super::read_write::{read, exists};

extern {
    
    fn bitmap_font(pixels: *mut u8, height: u32, width: u32);    
}

pub fn get_dict(content: &Content) -> /*Vec<Vec<String>>*/ Dict {

    let mut dict = Dict::new();

    //let mut dict: Vec<Vec<String>> = Vec::new();

    for line in content.get_content().split(constants::LINE_DELIMITER_SLASH_N).collect::<Vec<&str>>() {

        let mut key_value_pair: Vec<String> = Vec::new();
       
        for token in (line.split(constants::LINE_DELIMITER_SLASH_R).collect::<Vec<&str>>()[0]).split(constants::KEY_VALUE_DELIMITER).collect::<Vec<&str>>() {
            
            key_value_pair.push(token.to_string());
        }

        if key_value_pair.len() > 1 {

            //println!("key_value_pair_len = {}", key_value_pair.len());

            dict.update(key_value_pair[0].to_string(), key_value_pair[1].to_string());
        }

        //dict.push(key_value_pair);    
    }

    //let lines: Vec<&str> = content.get_content().split(constants::LINE_DELIMITER_SLASH_N).collect();

    //let header_and_body_vec: Vec<&str> = content.get_content().split(constants::END_OF_HEADER_MARKER).collect();

    /*
    let s: String = "hello: world\r\nbye: world\r\n".to_string();

    let ls = s.split(constants::LINE_DELIMITER_SLASH_N);

    for line in ls {
        
        let is = line.split(constants::LINE_DELIMITER_SLASH_R);

        for in_line in is {

            if in_line.len() > 0 {

                println!("---------------------------> {}", in_line);
            }
        }
    }
     */
    

    /*
    for line in content.get_content().split(constants::LINE_DELIMITER_SLASH_N).collect::<Vec<&str>>() {

        //println!("---> {}", line);

        //let local_line = line.split(constants::LINE_DELIMITER_SLASH_R).collect::<Vec<&str>>();

        //println!("-----> {}", local_line[0]);

        //println!("-----> {}", line.split(constants::LINE_DELIMITER_SLASH_R).collect::<Vec<&str>>()[0]);

        //let tokens = (line.split(constants::LINE_DELIMITER_SLASH_R).collect::<Vec<&str>>()[0]).split(constants::KEY_VALUE_DELIMITER).collect::<Vec<&str>>();

        for token in (line.split(constants::LINE_DELIMITER_SLASH_R).collect::<Vec<&str>>()[0]).split(constants::KEY_VALUE_DELIMITER).collect::<Vec<&str>>() {

            println!("-> {}", token);
        }



        //println!("------->>>>>> {}", line.split(constants::LINE_DELIMITER_SLASH_R).collect());

        
        //for in_line in line.split(constants::LINE_DELIMITER_SLASH_R) {

        //    println!("------> {}", in_line);
        //}
        
    }
     */
    
    // return 
    dict
}

fn get_header(mut stream: &TcpStream, get_body_as_well: bool) -> Dict {

    let mut buffer = [0; constants::SIZE_OF_SINGLE_READ];
    let mut content: Content;
    let mut full_buffer = Vec::<u8>::new();
    //let mut boundaries = Vec::new();

    content = Content {content: Vec::new(), content_length: 0};

     // This regex matches:
    // ^--[a-zA-Z0-9]+(--)?$
    // Where:
    // ^-- = starting boundary marker
    // [a-zA-Z0-9]+ = the actual boundary string
    // (--)? = optional closing marker
    //let re = Regex::new(r"(?m)^--[a-zA-Z0-9]+(--)$").unwrap();
    //let re = Regex::new(r"(?m)^--[a-zA-Z0-9]+--$").unwrap();
    let re = Regex::new(r"--AudioUploadBoundary\d+--\r\n$").unwrap();
    let re_for_data_html = Regex::new(r"(?m)Address.SoundFileName=").unwrap();

    loop {        
        match stream.read(&mut buffer) {

            Ok(size) => {

                //content = Content {content: buffer.to_vec(), content_length: size};
                //full_buffer.extend_from_slice(&buffer[..size]);

                /*println!("RAW BUFFER ({} bytes): {:?}", size, String::from_utf8_lossy(&buffer[..size]));*/

                /*if (size == 0)
                {
                    return Dict::new();
                }*/

                full_buffer.extend_from_slice(&buffer[..size]);                
            }

            Err(_e) => {

                return Dict::new();
            }
        }

        //println!("RAW BUFFER ({} bytes): {:?}", size, String::from_utf8_lossy(&buffer[..size]));

        /*for mat in re.find_iter(&full_buffer) {
            boundaries.push(mat.start());
        }*/

        if re.is_match(&full_buffer) {

            content = Content {content: full_buffer.clone(), content_length: full_buffer.len()};

            println!("{}", String::from_utf8_lossy(content.get_content().as_bytes()));

            /*println!("BROKEN BROKEN BROKEB");*/
            break;
        }

        //println!("Boundaries: {:?}", boundaries);
        //println!("{}", boundaries.len());    

        /*if boundaries.len() >= 1 {

            content = Content {content: full_buffer.clone(), content_length: full_buffer.len()};
            break;
        }*/

        if re_for_data_html.is_match(&full_buffer) {

            content = Content {content: full_buffer.clone(), content_length: full_buffer.len()};
            break;
        }

        /*for mat in re_for_data_html.find_iter(&full_buffer) {

            content = Content {content: full_buffer.clone(), content_length: full_buffer.len()};
            break;
        }*/

        // Check for header-body separator (double CRLF)
        /*if full_buffer.windows(4).any(|w| w == b"\r\n\r\n") {

            content = Content {content: full_buffer.clone(), content_length: full_buffer.len()};
            break;
        }*/
    }
        
    let mut dict = get_dict(&content);
   
    for line in content.get_content().split(constants::END_OF_SINGLE_HEADER_LINE_MARKER).collect::<Vec<&str>>() {

        let vec = line.split(&" ".to_string()).collect::<Vec<&str>>();

        if dict.len() > 0 {

            dict.update("METHOD".to_string(), vec[0].to_string());
            dict.update("RESOURCE_URI".to_string(), vec[1].to_string());
            dict.update("HTTP_VERSION".to_string(), vec[2].to_string());
            
            // Resource uri query parameters
            /*
            let rui = dict.find("RESOURCE_URI");
            let rui_clone = rui[1].clone();
             */
            let rui_clone = vec[1].to_string();
            let uri_query_parameters = rui_clone.split(&"?".to_string()).collect::<Vec<&str>>();
            
            if uri_query_parameters.len() > 1 {

                //dict.update("RESOURCE_URI_FILE".to_string(), uri_query_parameters[0].to_string());

                //println!("Resource URI Query Parameters are given = {}", uri_query_parameters[1]);
                dict.update("RESOURCE_QUERY_PARAMETERS".to_string(), uri_query_parameters[1].to_string());
            }
            /*
            else {

                println!("Length = {}, {}", uri_query_parameters.len(), uri_query_parameters[0]);
            }
             */
            dict.update("RESOURCE_URI_FILE".to_string(), uri_query_parameters[0].to_string());

        } 
        /*else {
            println!("dict length is 0");
        }*/

        break;
    }

    if !get_body_as_well {

        return dict
    }
    
    /*
        Now get body of this request
     */
    /*let body = content.get_content().split(constants::START_OF_BODY_MARKER).collect::<Vec<&str>>();
    if !(body.len() > 1) {

        return dict
    }*/

    let _number_of_bodyparts = content.get_content().split(constants::START_OF_BODY_MARKER).collect::<Vec<&str>>().len();

    if !(_number_of_bodyparts > 1) {

        return dict
    }

//println!("-->***********>>>>>>>>>>>> {}", content.get_content().split(constants::START_OF_BODY_MARKER).collect::<Vec<&str>>()[1..]);

    for bodypart in content.get_content().split(constants::START_OF_BODY_MARKER).collect::<Vec<&str>>().iter().skip(_number_of_bodyparts - 1) {

        dict.update("BODY".to_string(), bodypart.to_string());

        //println!("-> {}", dict.find("Content-Length")[1]);

        //println!("--> {}", String::from_utf8_lossy(bodypart.as_bytes()));
    }
            
    dict
}

pub fn handle_connection(mut stream: TcpStream, config_dict: &Dict) {

    let mut content = Content {content: Vec::new(), content_length: 0};

    unsafe { bitmap_font( core::ptr::null_mut(), 0, 0); }

    let document_root = config_dict.find("DocumentRoot");
    if !(document_root.len() > 1) {

        /*
            Send status code 500, Internal server error
         */
         content.set_content("<html><head><title>index.html</title></head><body><p>Internal server error.</p></body></html>");
         Write::write_all(&mut stream, ("HTTP/1.1 500 OK\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap(); 

         return
    }                
        
    let header_dict = get_header(&stream, true);
                
    let keys = header_dict.keys();
    let values = header_dict.values();

    
    //println!("Keys: {:?}, Values: {:?}", keys, values);


    if keys.len() != values.len() {
                
        /*
            Send status code 500, Internal server error
         */
        content.set_content("<html><head><title>index.html</title></head><body><p>Internal server error.</p></body></html>");               
        Write::write_all(&mut stream, ("HTTP/1.1 500 OK\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();

        /*
        for i in 0 .. keys.len() {
    
            println!("key = {} : value = {}", keys[i], values[i]);
        }
        */
                
        return
    }
    
    let method = header_dict.find("METHOD");

    if !(method.len() > 0) {

        /*
            Send status code 400, Bad request            
         */
        content.set_content("<html><head><title>index.html</title></head><body><p>Method not found.</p></body></html>");
        Write::write_all(&mut stream, ("HTTP/1.1 400 OK\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();

        return
    }

    match method[1].as_str() {

        "GET" => {

            // Handle
            /* Check for resource-uri, send reply based on the resource-uri */ 
            
            /*
                let resource_uri = header_dict.find("RESOURCE_URI");
             */
            let resource_uri_file = header_dict.find("RESOURCE_URI_FILE");

            if !(resource_uri_file.len() > 1) {

                /*
                    Send status code 400, Bad request
                 */                                
                return
            }

            // Now deal with end-points
            match resource_uri_file[1].to_lowercase().as_str() {

                "/favicon.ico" => {
                    if exists(document_root[1].to_string().as_str()) {

                        content = read((document_root[1].to_string() + "/favicon.ico").as_str());
                        Write::write_all(&mut stream, ("HTTP/1.1 200 OK\r\nContent-Type: image/x-icon\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();
                    }
                    else {

                        content.set_content("<html><head><title>index.html</title></head><body><p>Internal server error.</p></body></html>");               
                        Write::write_all(&mut stream, ("HTTP/1.1 500 OK\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();
                    }
                }

                "/" | "/index.html" => {
                    if exists(document_root[1].to_string().as_str()) {
                                           
                         content = read((document_root[1].to_string() + "/index.html").as_str());
                         Write::write_all(&mut stream, ("HTTP/1.1 200 OK\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();
                    }
                    else {

                        content.set_content("<html><head><title>index.html</title></head><body><p>Internal server error.</p></body></html>");               
                        Write::write_all(&mut stream, ("HTTP/1.1 500 OK\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();
                    } 
                }
                                                                              
                // When end-point is not available
                _ => {

                    /*
                    let resource_query_parameters = header_dict.find("RESOURCE_QUERY_PARAMETERS");                    
                    let resourec_query_parameters_clone = resource_query_parameters[1].clone();
                    let key_vec = resourec_query_parameters_clone.split(&"=".to_string()).collect::<Vec<&str>>();

                    if key_vec.len() > 1 {

                        content.set_content(&key_vec[1]);
                    }
                    else {

                        content.set_content("<p>I'm sorry, the content you requested is currently on a top secret mission for the government, we can't reveal its current location but we can tell you that it's saving the world one byte at a time &#x1F609;</p>");
                    }
                     */
                    /*
                        I got org.json.JSONException: value ... of type java.lang.String cannot be converted to JSONArray.
                        That is why the following line go commented and the same line is next converted as it is right after the commented line.
                     */
                    //content.set_content("<p>I'm sorry, the content you requested is currently on a top secret mission for the government, we can't reveal its current location but we can tell you that it's saving the world one byte at a time &#x1F609;</p>");
                    content.set_content("[\"I'm sorry. The content you requested is currently on a top secret mission for the government. We can't reveal its current location but we can tell you that it's saving the world one byte at a time.\"]");
                    Write::write_all(&mut stream, ("HTTP/1.1 200 OK\r\nConnection: Close\r\n".to_string() + "Content-Type: text/html\r\n" + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();
                }
            }
        }

        "POST" => {

            let resource_uri = header_dict.find("RESOURCE_URI");

            if !(resource_uri.len() > 1) {

                /*
                    Send status code 400, Bad request
                 */
                
                return
            }

            // Handle
            match resource_uri[1].to_lowercase().as_str() {

                "/display.html" => {

                    let _body = header_dict.find("BODY");

                    //println!("DISPLAY IS ..... {} = {}", _body[0], _body[1]);

                    content.set_content(""); 
                    Write::write_all(&mut stream, ("HTTP/1.1 200 OK\r\nContent-Type: text/javascript\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();
                }

                "/graphpaper.html" => {

                    let _body = header_dict.find("BODY");

                    //println!("graphpaper ..... {} = {}", _body[0], _body[1]);
                    
                    /*
                    png.new();
                     */

                    //content = read((document_root[1].to_string() + "/continue.html").as_str()); 
                    content.set_content("<p>The method you requested is currently being held captive by a group of mischievous monkeys, we're working on a rescue mission, try again later with a different method &#x1F609;</p>"); 
                    Write::write_all(&mut stream, ("HTTP/1.1 200 OK\r\nConnection: Close\r\n".to_string() + "Content-Type: text/html\r\nContent-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap(); 
                }

                "/data.html" => {

                    //println!("{} = {}", _body[0], _body[1]);

                    let _keys = header_dict.keys();
                    let _values = header_dict.values();

                    //let _body = header_dict.find("BODY");
                    //let _ct = header_dict.find("Content-Type");
                    //let _cl = header_dict.find("Content-Length");

                    //String::from_utf8_lossy(content.get_content().as_bytes())

                    _keys.iter().for_each(|key| {
                        println!("{} = {}", key, String::from_utf8_lossy(header_dict.find(key)[1].as_bytes()));
                    });

                    content.set_content("[\"data received.\"]");
                    Write::write_all(&mut stream, ("HTTP/1.1 200 OK\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();
                }

                "/multipart.html" => {

                    let _keys = header_dict.keys();
                    let _values = header_dict.values();

                    //let _body = header_dict.find("BODY");
                    //let _ct = header_dict.find("Content-Type");
                    //let _cl = header_dict.find("Content-Length");

                    //String::from_utf8_lossy(content.get_content().as_bytes())

                    _keys.iter().for_each(|key| {
                        println!("{} = {}", key, String::from_utf8_lossy(header_dict.find(key)[1].as_bytes()));
                    }); 
                                        
                    //println!("{} = {}", _ct[0], _ct[1]);
                    //println!("{} = {}", _cl[0], _cl[1]);
                    
                    // Parse the body
                                        
                    content.set_content("[\"multipart getting through.\"]");
                    Write::write_all(&mut stream, ("HTTP/1.1 200 OK\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();                    
                }

                _ => {

                    // Handle unknown resource uri

                    let _body = header_dict.find("BODY");

                    println!("DISPLAY IS..... {} = {}", _body[0], _body[1]);

                    content.set_content("<p>The method you requested is currently being held captive by a group of mischievous monkeys, we're working on a rescue mission, try again later with a different method &#x1F609;</p>"); 
                    //content.set_content("[\"I'm sorry. The content you requested is currently on a top secret mission for the government. We can't reveal its current location but we can tell you that it's saving the world one byte at a time.\"]");
                    Write::write_all(&mut stream, ("HTTP/1.1 200 OK\r\nConnection: Close\r\n".to_string() + "Content-Length: ".to_string().as_str() + content.get_content_length().to_string().as_str() + "\r\n\r\n" + content.get_content()).as_bytes()).unwrap();
                }
            }
        }

        "DELETE" => {

            // Handle
        }

        "PUT" => {

        }

        _ => {

            // Handle unknown method

        }        
    }

    /* Native function calls detail  */
    //unsafe {image(10)};
    
}