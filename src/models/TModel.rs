pub trait TModel<TInput, TOutput> {
    fn get_model(input: TInput) -> rusqlite::Result<Vec<TOutput>, rusqlite::Error>;
}
