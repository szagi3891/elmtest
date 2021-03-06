extern crate hyper;
extern crate getopts;
extern crate rustc_serialize;
extern crate crypto;
extern crate chrono;

mod main_cli_args;
mod server_app;
mod lib;
mod routing;
//mod test;

fn main() {

    match main_cli_args::get() {
        Ok((data_path, static_path)) => {
            
            /*
            test::blob_test::test(data_path);
            println!("koniec testu, udany, wychodzę");
            return;
            */

            println!("server start -> 0.0.0.0:8888");
            server_app::start_server(data_path, static_path);
        },
        
        Err(message) => {
            panic!("error getting cli args -> {:?}", message);
        }
    }
}
