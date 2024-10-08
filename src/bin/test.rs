use tokio::sync::Mutex;
use std::sync::Arc;

// #[tokio::main]
// async fn main() {
//     let data1 = Arc::new(Mutex::new(0));
//     let data2 = Arc::clone(&data1);
// 
//     tokio::spawn(async move {
//         let mut lock = data2.lock().await;
//         *lock += 1;
//     });
// 
//     let mut lock = data1.lock().await;
//     *lock += 1;
// }


#[tokio::main]
async fn main() {
    let count = Arc::new(Mutex::new(0));

    for i in 0..5 {
        let my_count = Arc::clone(&count);
        tokio::spawn(async move {
            for j in 0..10 {
                let mut lock = my_count.lock().await;
                *lock += 1;
                println!("{} {} {}", i, j, lock);
            }
        });
    }

    loop {
        if *count.lock().await >= 50 {
            break;
        }
    }
    println!("Count hit 50.");
}