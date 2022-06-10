use async_once::AsyncOnce;
use lazy_static::lazy_static;

lazy_static! {
    static ref FOO: AsyncOnce<u32> = AsyncOnce::new(
        async {
            1
        }
    );
    
    // I believe the issus lies either in jwt-simple or async_once 
    // because this still compiles if i comment out the above code while including jwt-simple in my Cargo.toml
    static ref BAR: u32 = 2;
}

fn main() {
    println!("adding jwt-simple = \"0.11\"! to the Cargo.toml breaks the compilation");
}
