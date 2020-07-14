
fn main() {
    struct User{
        username : String,
        email: String,
        sign_in_count : u64,
        active : bool,
    }


    let user = User{
        email : "www.baidu.com",
        username : "jc",
        sign_in_count : 22,
        active : true,
    };
    println!("user.email={}",user.email);
}
