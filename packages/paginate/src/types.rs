pub struct PaginationResponse<T> {
    pub data: Vec<T>,
    pub metadata: Metadata,
}

pub struct Metadata {
    pub has_more: bool,
}
