#[macro_export]
macro_rules! get_current_thread_id {
    () => {
        o!("thread-id" => format!("{:?}", thread::current().id()))
    }
}