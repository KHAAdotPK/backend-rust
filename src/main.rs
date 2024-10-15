/*
    src/main.rs
    aEon@khaa.pk
 */

pub const CONFIG_FILE: &'static str = r".\conf\httpd.conf";
pub const LOCAL_SOCKET: &'static str = r"192.168.100.17:3968";

extern crate khaa_pk;

fn main() {
    
    // ---------------------
    // | Start HTTP server |
    // ---------------------
    
    khaa_pk::start_server(CONFIG_FILE);     
}



