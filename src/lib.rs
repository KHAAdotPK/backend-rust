/*
    src/lib.rs
    aEon@khaa.pk
 */

mod modules;

use std::{net::TcpListener,};
use modules::{khaa_pk_read_write::sundry::{get_dict, self}, model::dict::DictBody, constants};

pub fn start_server(conf_file: &'static str) {
         
    let content = modules::khaa_pk_read_write::read_write::read(conf_file);
                    
    let config_dict = get_dict(&content);
        
    let vec = config_dict.find(constants::CONFIG_LOCAL_SOCKET_TOKEN);
    
    let listener = TcpListener::bind(vec[1].to_string());
        
    match listener {
       
        Ok(l) => {

            for s in l.incoming() {

                match s {

                    Ok(stream) => {
                        
                        sundry::handle_connection(stream, &config_dict);
                    }

                    Err(e) => {
                                                
                        panic!("Failed with error {}", e);
                    }    
                }
            }            
        }

        Err(e) => {

            panic!("Failed with error {}", e);
        }
    } 
                
}