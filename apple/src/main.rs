const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // 居中显示信息
    let mut welcome_msg = format!(":j-editor 🎈🎉✨🎊🎇🎈 -- version {} \r", VERSION);
    let width = 50 as usize;
    let msg_len = welcome_msg.len();
    let padding_len = width.saturating_sub(msg_len) / 2;
    let padding = " ".repeat(padding_len.saturating_sub(1));
    welcome_msg = padding + welcome_msg.as_str();
    // truncate保证消息不会超过窗口宽度
    welcome_msg.truncate(width);
    println!("{}\r", welcome_msg);
}
