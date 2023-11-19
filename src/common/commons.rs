use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Clone, Display, EnumString)]
pub enum SortOrder {
    #[default]
    Asc,
    Desc,
}
