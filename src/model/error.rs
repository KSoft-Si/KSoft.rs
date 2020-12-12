use serde::Deserialize;

pub struct Error404 {
    pub code: Option<u16>,
    pub error: Option<bool>,
    pub exists: Option<bool>,
    pub message: Option<String>,
    pub cache: Option<bool>,
    pub total: Option<i32>,
    pub took: Option<i32>,
    pub data: Option<Vec<crate::music::Lyrics>>,
    pub voted: Option<bool>
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawError {
    pub code: Option<u16>,
    pub error: Option<bool>,
    pub exists: Option<bool>,
    pub details: Option<String>,
    pub message: Option<String>,
    pub cache: Option<bool>,
    pub total: Option<i32>,
    pub took: Option<i32>,
    pub data: Option<Vec<crate::music::Lyrics>>,
    pub voted: Option<bool>
}

pub struct Error400 {
    pub error: bool,
    pub code: u16,
    pub message: String
}

pub struct Error409 {
    pub code: u16,
    pub error: bool,
    pub exists: bool,
    pub message: String
}

pub struct Error429 {
    pub details: String
}

pub trait ApiError {}
impl ApiError for Error404 {}
impl ApiError for Error400 {}
impl ApiError for Error409 {}
impl ApiError for Error429 {}

pub enum SpecificError {
    RecognizedError(Box<dyn ApiError>),
    UnrecognizedError
}

impl RawError {
    pub fn specific(self) -> SpecificError {
        return match &self.code {
            None => SpecificError::UnrecognizedError,
            Some(c) => {
                match c {
                    400u16 => {
                        SpecificError::RecognizedError(Box::new(Error400 {
                            code: self.code.unwrap(),
                            error: self.error.unwrap(),
                            message: self.message.unwrap()
                        }))
                    },
                    401u16 => {
                        SpecificError::RecognizedError(Box::new(Error400 {
                            code: self.code.unwrap(),
                            error: self.error.unwrap(),
                            message: self.message.unwrap()
                        }))
                    }
                    402u16 => {
                        SpecificError::RecognizedError(Box::new(Error400 {
                            code: self.code.unwrap(),
                            error: self.error.unwrap(),
                            message: self.message.unwrap()
                        }))
                    },
                    409u16 => {
                        SpecificError::RecognizedError(Box::new(Error409 {
                            code: self.code.unwrap(),
                            error: self.error.unwrap(),
                            exists: self.exists.unwrap(),
                            message: self.message.unwrap()
                        }))
                    },
                    429u16 => {
                        SpecificError::RecognizedError(Box::new(Error429 {
                            details: self.details.unwrap()
                        }))
                    }
                    _ => { SpecificError::UnrecognizedError }
                }
            }
        }
    }
}