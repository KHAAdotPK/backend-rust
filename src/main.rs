/*
    src/main.rs
    Q@khaa.pk
 */

pub const CONFIG_FILE: &'static str = r".\conf\httpd.conf";
pub const LOCAL_SOCKET: &'static str = r"192.168.100.17:3968";

extern crate khaa_pk;

use argsv::{start, find_arg, stop, help, help_line, common_argc, process_argument};

fn main() {

    let command_lines = "h -h help --help ? /? (Displays help screen)\n\
                        v -v version --version /v (Displays version number)\n\
                        verbose --verbose (Displays detailed information or feedback about the execution of another command)\n";

    // Get the command-line arguments as an iterator
    let args: Vec<String> = std::env::args().collect();
    // Create a Vec<CString> from the Vec<String>
    let c_args: Vec<std::ffi::CString> = args.iter().map(|s| std::ffi::CString::new(s.as_str()).expect("Failed to create CString")).collect();
    // Get the equivalent of argv in C. Create a Vec<*const c_char> from the Vec<CString>.
    let c_argv: Vec<*const std::os::raw::c_char> = c_args.iter().map(|s| s.as_ptr()).collect();
    // Get the C equivalent of argc.    
    let argc: i32 = c_args.len() as std::os::raw::c_int;

    let head = start (argc, c_argv, command_lines);

    println!("argc: {}", argc);

    let arg_help = find_arg (head, command_lines, "h");
    if !arg_help.is_null() || argc < 2 {
        help (head, command_lines);
        stop (head); 
        return;
    }
    
    stop(head);
    
    // ---------------------
    // | Start HTTP server |
    // ---------------------
    
    khaa_pk::start_server(CONFIG_FILE);     
}



