use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct PaginationResponse<T> {
    pub data: Vec<T>,
    pub metadata: Metadata,
}

#[cw_serde]
pub struct Metadata {
    pub has_more: bool,
}
