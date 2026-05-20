pub trait ValidateObject {
    type Output;
    type Error;

    fn try_validate(self) -> Result<Self::Output, Self::Error>;
}
