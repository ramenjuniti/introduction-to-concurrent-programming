use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let val = Arc::new(RwLock::new(true));

    // let t = thread::spawn(move || {
    //     let flag = val.read().unwrap();
    //     if *flag {
    //         *val.write().unwrap() = false;
    //         println!("flag is true");
    //     }
    // });

    // let t = thread::spawn(move || {
    //     let flag = *val.read().unwrap();
    //     if flag {
    //         *val.write().unwrap() = false;
    //         println!("flag is true");
    //     }
    // });

    // let t = thread::spawn(move || {
    //     let _flag = val.read().unwrap();
    //     *val.write().unwrap() = false;
    //     println!("deadlock");
    // });

    let t = thread::spawn(move || {
        let _ = val.read().unwrap();
        *val.write().unwrap() = false;
        println!("not deadlock");
    });

    t.join().unwrap();
}