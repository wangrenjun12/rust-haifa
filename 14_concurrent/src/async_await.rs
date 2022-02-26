use std::future::Future;
use futures::executor::block_on;


async fn do_something() {
    println!("HaHa")
}

fn main(){
    let future =  do_something();
    block_on(future);

}
