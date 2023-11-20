use async_graphql::Enum;
use poem_openapi::Enum as OpenApiEnum;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    Debug,
    Enum,
    OpenApiEnum,
    Copy,
    Clone,
    Default,
    Display,
    EnumString,
    Deserialize,
    Serialize,
    Eq,
    PartialEq,
)]
pub enum SortOrder {
    #[default]
    Asc,
    Desc,
}

#[derive(Debug, Clone, Display, EnumString)]
pub enum SQLComparison<T>
where
    T: std::default::Default,
{
    Equal(T),
    NotEqual(T),
    GreaterThan(T),
    GreaterThanOrEqual(T),
    LessThan(T),
    LessThanOrEqual(T),
    Like(T),
    NotLike(T),
    In(T),
    NotIn(T),
    IsNull(T),
    IsNotNull(T),
}
