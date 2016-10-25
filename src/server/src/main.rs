extern crate hyper;
extern crate getopts;

mod main_cli_args;
mod server_app;
mod lib;
mod routing;
mod outresponse;

fn main() {

    match main_cli_args::get() {
        Ok((data_path, static_path)) => {
            
            println!("server start -> 0.0.0.0:8888");
            server_app::start_server(data_path, static_path);
        },
        
        Err(message) => {
            panic!("error getting cli args -> {:?}", message);
        }
    }
}
