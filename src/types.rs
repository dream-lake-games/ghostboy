use crate::prelude::*;

pub trait Queryable:
    Sized
    + Clone
    + Send
    + Sync
    + 'static
    + std::fmt::Debug
    + Reflect
    + FromReflect
    + TypePath
    + GetTypeRegistration
{
}
