use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Clone, Display, EnumString)]
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
