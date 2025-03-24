use references::{
    check::errors::Error as RefCheckErr, eval::errors::Error as RefEvalErr,
    parser::errors::Error as RefParseErr,
};
use std::{fmt, io::Error as IOError};
use stlc::{
    check::errors::Error as StlcCheckErr, eval::errors::Error as StlcEvalErr,
    eval_context::errors::Error as StlcEvalCtxErr, parser::errors::Error as StlcParseErr,
};
use untyped_arithmetic::parse::errors::Error as ArithErr;
use untyped_lambda::parse::errors::Error as LamErr;

#[derive(Debug)]
pub enum Error {
    IO(IOError),
    UntypedArith(ArithErr),
    UntypedLam(LamErr),
    StlcParse(StlcParseErr),
    StlcCheck(StlcCheckErr),
    StlcEval(StlcEvalErr),
    StlcEvalCtx(StlcEvalCtxErr),
    RefParse(RefParseErr),
    RefCheck(RefCheckErr),
    RefEval(RefEvalErr),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO error: {err}"),
            Error::UntypedArith(err) => err.fmt(f),
            Error::UntypedLam(err) => err.fmt(f),
            Error::StlcParse(err) => err.fmt(f),
            Error::StlcCheck(err) => err.fmt(f),
            Error::StlcEval(err) => err.fmt(f),
            Error::StlcEvalCtx(err) => err.fmt(f),
            Error::RefParse(err) => err.fmt(f),
            Error::RefCheck(err) => err.fmt(f),
            Error::RefEval(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IO(err)
    }
}

impl From<ArithErr> for Error {
    fn from(err: ArithErr) -> Error {
        Error::UntypedArith(err)
    }
}

impl From<LamErr> for Error {
    fn from(err: LamErr) -> Error {
        Error::UntypedLam(err)
    }
}

impl From<StlcParseErr> for Error {
    fn from(err: StlcParseErr) -> Error {
        Error::StlcParse(err)
    }
}

impl From<StlcCheckErr> for Error {
    fn from(err: StlcCheckErr) -> Error {
        Error::StlcCheck(err)
    }
}

impl From<StlcEvalErr> for Error {
    fn from(err: StlcEvalErr) -> Error {
        Error::StlcEval(err)
    }
}

impl From<StlcEvalCtxErr> for Error {
    fn from(err: StlcEvalCtxErr) -> Error {
        Error::StlcEvalCtx(err)
    }
}

impl From<RefParseErr> for Error {
    fn from(err: RefParseErr) -> Error {
        Error::RefParse(err)
    }
}

impl From<RefCheckErr> for Error {
    fn from(err: RefCheckErr) -> Error {
        Error::RefCheck(err)
    }
}

impl From<RefEvalErr> for Error {
    fn from(err: RefEvalErr) -> Error {
        Error::RefEval(err)
    }
}
