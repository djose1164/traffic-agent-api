use super::database::DbError;

pub trait Repository {
    type T;

    fn read(&self, identifier: u32) -> Result<Self::T, DbError>; 
}