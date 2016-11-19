pub enum IonodeError {
    NoFile(String),
}

pub type IonodeResult<T> = Result<T, IonodeError>;