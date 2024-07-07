use futures::{future, future::BoxFuture, stream, FutureExt, StreamExt}; // 0.3.13
use std::time::{Duration, Instant};
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    //let now = Instant::now();
    loop {
        let batch_of_tasks = future::join_all(all_tasks());
        future::join(batch_of_tasks, time::sleep(Duration::from_secs(5))).await;
    }

    /* 
    let forever = stream::unfold((), |()| async {
        eprintln!("Loop starting at {:?}", Instant::now());

        // Resolves when all pages are done
        let batch_of_tasks = future::join_all(all_tasks());

        // Resolves when both all pages and a delay of 1 second is done
        future::join(batch_of_tasks, time::sleep(Duration::from_secs(5))).await;
        
        Some(((), ()))
    });
    forever.take(5).for_each(|_| async {}).await;*/
    //eprintln!("Took {:?}", now.elapsed());
    Ok(())    
}

fn all_tasks() -> Vec<BoxFuture<'static, ()>> {
    vec![processshift().boxed()]
}

async fn processshift() {
    
    println!("called processsshift");
    let input = rtftp::getmessage("one").unwrap();

    let processed = ddshift::transform(input.as_str());

    let struct_str = format!("{:#?}", processed);
    println!("{}", struct_str);
    let _ = rbmq::send_msg(&struct_str);
        
}