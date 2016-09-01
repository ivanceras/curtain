use rustorm::database::DbError;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use rustc_serialize::json;


#[derive(Debug)]
pub enum ServiceError{
    Error(String),// generic service error
    DbError(DbError), // db related error
    ParseError(ParseError), // json parsing, inquerest parsing error
    ParamError(ParamError), // parameter missing errors
}

impl ServiceError{

    pub fn new(description: &str) -> Self {
        ServiceError::Error(description.to_owned())
    }
}


impl Error for ServiceError{
    
    fn description(&self) -> &str{
        match *self{
            ServiceError::Error(ref description) => description,
            ServiceError::DbError(ref err) => err.description(),
            ServiceError::ParseError(ref err) => err.description(),
            ServiceError::ParamError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self{
            ServiceError::Error(_) => None,
            ServiceError::DbError(ref err) => Some(err),
            ServiceError::ParseError(ref err) => Some(err),
            ServiceError::ParamError(ref err) => Some(err),
        }
    }
}

impl Display for ServiceError{
   
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            ServiceError::Error(ref err) => write!(f, "{}", self.description()),
            ServiceError::DbError(ref err) => write!(f, "Db Error: {}", err),
            ServiceError::ParseError(ref err) => write!(f, "Parse error: {}", err),
            ServiceError::ParamError(ref err) => write!(f, "Param error: {}", err)
        }
    }
}

impl From<DbError> for ServiceError{
    
    fn from(err: DbError) -> Self{
        ServiceError::DbError(err)
    }
}

impl From<ParseError> for ServiceError{
    
    fn from(err: ParseError) -> Self{
        ServiceError::ParseError(err)
    }
}

impl From<ParamError> for ServiceError{
    fn from(err: ParamError) -> Self{
        ServiceError::ParamError(err)
    }
}


#[derive(Debug)]
pub enum ParseError{
    Error(String),// parsing inquerest error
    JsonParserError(json::ParserError),// json parsing error
}


impl ParseError{
	
	pub fn new(description: &str)->Self{
		ParseError::Error(description.to_owned())
	}
}

impl fmt::Display for ParseError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            ParseError::Error(_) => write!(f, "{}", self.description()),
            ParseError::JsonParserError(ref err) => write!(f, "Json parsing error {}", err),
        }
    }
}

impl Error for ParseError{
    
    fn description(&self) -> &str{
        match *self{
            ParseError::Error(ref description) => description,
            ParseError::JsonParserError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self{
            ParseError::Error(_) => None,
            ParseError::JsonParserError(ref err) => Some(err)
        }
    }
}

impl From<json::ParserError> for ParseError{
    
    fn from(err: json::ParserError) -> Self{
        ParseError::JsonParserError(err)
    }
}


#[derive(Debug)]
pub enum ParamError{
    Error(String)
}

impl ParamError{
    pub fn new(description: &str) -> Self {
        ParamError::Error(description.to_owned())
    }
}

impl fmt::Display for ParamError{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self{
            ParamError::Error(_) => write!(f, "{}", self.description())
        }
    }
}

impl Error for ParamError{
    fn description(&self) -> &str{
        match *self{
            ParamError::Error(ref description) => description,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self{
            ParamError::Error(_) => None
        }
    }
}

