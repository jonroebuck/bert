pub async fn run() {
    let _bootstrap_context = crate::bootstrap::loader::load().await;
    crate::bootstrap::context::assemble();
}
