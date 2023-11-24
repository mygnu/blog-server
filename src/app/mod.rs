pub(crate) mod posts;

/// A helper function to run a blocking function in a threadpool and tracing
/// span
pub(crate) async fn tracing_block<F, I>(
    f: F,
) -> std::result::Result<I, actix_web::error::BlockingError>
where
    F: FnOnce() -> I + Send + 'static,
    I: Send + 'static,
{
    let current_span = tracing::Span::current();
    actix_web::web::block(move || current_span.in_scope(f)).await
}
