#[cfg(feature = "runtime")]
#[test]
fn runtime() {
    println!("{:?}", geekera::runtime::init());
    geekera::runtime::get().unwrap().block_on(async {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        println!("OK");
    });
}
