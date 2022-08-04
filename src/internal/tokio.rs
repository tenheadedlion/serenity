use pink_sidevm as sidevm;
use std::future::Future;

pub fn spawn_named<F, T>(_name: &str, future: F) -> sidevm::task::JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    sidevm::spawn(future)
}
