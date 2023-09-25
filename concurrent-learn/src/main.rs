#[allow(unused)]
mod msg_passing;
mod mutex;
mod spawn_basic;
fn main() {
    // spawn_basic::spawn_demo();
    // msg_passing::msg_passing_demo();
    // msg_passing::msg_passing_multi_value();
    // msg_passing::msg_passing_multi_producer();
    // mutex::mutex_demo();
    mutex::arc_demo();
}
