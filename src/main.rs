use async_once::AsyncOnce;
use lazy_static::lazy_static;

lazy_static! {
    static ref FOO: AsyncOnce<u32> = AsyncOnce::new(
        async {
            1
        }
    );
}

fn main() {
    println!("adding jwt-simple = \"0.11\"! to the Cargo.toml breaks the compilation");
}
