mod password;

use password::create_pass;

fn main(){
    let password: String = create_pass(32, true, true);
    println!("{}", password)
}