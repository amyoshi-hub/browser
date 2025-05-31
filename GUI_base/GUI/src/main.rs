extern "C" {
    fn run_glfw_app() -> i32;
}

fn main() {
    unsafe {
        let ret = run_glfw_app();
        println!("C++側の戻り値: {}", ret);
    }
}

