pub trait TModel<TInput, TOutput> {
    fn get_model(input: &mut TInput) -> rusqlite::Result<Vec<TOutput>, rusqlite::Error>;
    fn insert(input: &mut TInput) -> rusqlite::Result<bool, rusqlite::Error>;
    fn update(input: &mut TInput) -> rusqlite::Result<bool, rusqlite::Error>;
    fn delete(input: &mut TInput) -> rusqlite::Result<bool, rusqlite::Error>;
}
