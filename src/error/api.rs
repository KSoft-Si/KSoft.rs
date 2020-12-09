use crate::model::error::*;

pub enum ResponseError {
    E404(Error404)
}