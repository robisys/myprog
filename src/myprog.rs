
//
#[cfg(unix)]
fn is_executable() {
   println!("is unix");
   }
#[cfg(windows)]
fn is_executable() {
   println!("is windows");
   }   

fn main() {
is_executable();
println!("Hallo World");
}
#[test]
fn its_really_works() {
}
