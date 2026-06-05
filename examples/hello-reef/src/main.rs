use std::env;


fn main(){

    for env_ in env::vars(){
        println!("{:?}", env_);
    }
}