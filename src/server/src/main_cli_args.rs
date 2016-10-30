use getopts::Options;
use std::env;
use std::collections::HashMap;
use std::env::current_dir;

pub fn get() -> Result<(String, HashMap<String, String>), String> {

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt("d", "data", "set path of data", "DATA_PATH");
    opts.optmulti("s", "static", "set path of static", "STATIC_PATH");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            return Err(f.to_string())
        }
    };
    
    let data_param = match matches.opt_str("data") {
        Some(path) => path,
        None => return Err("Required option 'data'".into()),
    };
    
    let mut data_path = current_dir().unwrap();
    
    data_path.push(data_param);
    
    let static_list = matches.opt_strs("static");

    /*
    Split by char:
    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

    Split by string:
    let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
    assert_eq!(v, ["lion", "tiger", "leopard"]);

    Split by closure:
    let v: Vec<&str> = "abc1def2ghi".split(|c: char| c.is_numeric()).collect();
    assert_eq!(v, ["abc", "def", "ghi"]);
    */
    
    let mut static_path = HashMap::new();
    
    for static_item in &static_list {
        let parts: Vec<&str> = static_item.split('=').collect();
        
        if parts.len() == 2 {
            let key: String = parts[0].into();
            let value = parts[1].into();
            
            if key == "api" {
                return Err(format!("static param -> reserved value api: {:?}", static_item));
            }
            
            let key_message = key.clone();
            
            let prev_value = static_path.insert(key, value);
            if let Some(_) = prev_value {
                return Err(format!("static params -> duplicate prefix {:?}", key_message));
            }

        } else {
            return Err(format!("static param -> incorect separator: {:?}", static_item));
        }
    }
    
    Ok((data_path.to_str().unwrap().to_string(), static_path))
}