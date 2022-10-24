pub mod character;

trait Repository {
    fn get_all() -> Result<Self, Box<dyn std::error::Error>>;
    fn get_by_id() -> Self;
    fn get_multiple() -> Vec<Self>;
}
