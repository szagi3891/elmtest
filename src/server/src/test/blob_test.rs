use std::path::PathBuf;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use lib::blob_stor::BlobStor;


pub fn test(path: PathBuf) {

    //let mut stor = BlobStor::new(path, 1000);
    let mut stor = BlobStor::new(path, 10);

    let text1 = "dsadasdsa dasdasdas dasdasd aaa 111";
    let text2 = "dsadasdsa dasdasdas dasdasd aaa 222";
    let text3 = "dsadasdsa dasdasdas dasdasd aaa 333";
    let text4 = "dsadasdsa dasdasdas dasdasd aaa 444";
    let text5 = "dsadasdsa dasdasdas dasdasd aaa 555";

    set(&mut stor, text1);
    set(&mut stor, text2);
    set(&mut stor, text3);
    set(&mut stor, text4);
    set(&mut stor, text5);
    
    for i in 1..10000 {
        let text = format!("automatyczny {:?}", i);
        set(&mut stor, text.as_str());
    }
    
    if stor.get("fc401e452f718439191c4fa43262d2e0024871cb".as_bytes()).unwrap().as_slice() == "dsadasdsa dasdasdas dasdasd aaa 222".as_bytes() {
        println!("pobranie ok");
    } else {
        panic!("problem z pobraniem");
    }
}

fn set(stor: &mut BlobStor, text: &str) {

    // create a Sha1 object
    let mut hasher = Sha1::new();
    // write input message
    hasher.input_str(text);
    // read hash digest
    let hex = hasher.result_str();
    //assert_eq!(hex, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");

    stor.set(hex.as_bytes(), text.as_bytes());
}
