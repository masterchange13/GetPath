use std::env;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    match env::current_dir() {
        Ok(path) => {
            let path_str = path.display().to_string();
            println!("当前路径: {}", path_str);

            // 将路径复制到剪贴板
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(path_str).unwrap();

            println!("路径已复制到剪贴板！");
        }
        Err(e) => println!("无法获取当前路径: {}", e),
    }
}
