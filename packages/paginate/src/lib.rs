use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Order, StdError, StdResult, Storage};
use cw_storage_plus::{Bound, IndexList, IndexedMap, KeyDeserialize, Map, PrimaryKey};
use serde::{de::DeserializeOwned, ser::Serialize};

#[cw_serde]
pub struct PaginationResponse<T> {
    pub data: Vec<T>,
    pub metadata: Metadata,
}

#[cw_serde]
pub struct Metadata {
    pub has_more: bool,
}

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 30;

/// Collect items in an iterator into a Vec.
///
/// For each item, apply a mutation as defined by `parse_fn`. This is useful if
/// the contract wants to convert the {key, value} pairs into a response type.
pub fn collect<'a, D, T, R, E, F>(
    iter: Box<dyn Iterator<Item = StdResult<(D, T)>> + 'a>,
    limit: Option<u32>,
    parse_fn: F,
) -> Result<Vec<R>, E>
where
    F: Fn(D, T) -> Result<R, E>,
    E: From<StdError>,
{
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    iter.take(limit)
        .map(|item| {
            let (k, v) = item?;
            parse_fn(k, v)
        })
        .collect()
}

/// Iterate entries in a `cw_storage_plus::Map`.
///
/// TODO: add docs
pub fn paginate_map<'a, K, T, R, E, F>(
    map: &Map<K, T>,
    store: &dyn Storage,
    start: Option<Bound<'a, K>>,
    limit: Option<u32>,
    parse_fn: F,
) -> Result<Vec<R>, E>
where
    K: PrimaryKey<'a> + KeyDeserialize,
    K::Output: 'static,
    T: Serialize + DeserializeOwned,
    F: Fn(K::Output, T) -> Result<R, E>,
    E: From<StdError>,
{
    let iter = map.range(store, start, None, Order::Ascending);
    collect(iter, limit, parse_fn)
}

/// Iterate entries in a `cw_storage_plus::Map` under a given prefix.
///
/// TODO: add docs
pub fn paginate_map_prefix<'a, K, T, R, E, F>(
    map: &Map<K, T>,
    store: &dyn Storage,
    prefix: K::Prefix,
    start: Option<Bound<'a, K::Suffix>>,
    limit: Option<u32>,
    parse_fn: F,
) -> Result<Vec<R>, E>
where
    K: PrimaryKey<'a>,
    K::Suffix: PrimaryKey<'a> + KeyDeserialize,
    <K::Suffix as KeyDeserialize>::Output: 'static,
    T: Serialize + DeserializeOwned,
    F: Fn(<K::Suffix as KeyDeserialize>::Output, T) -> Result<R, E>,
    E: From<StdError>,
{
    let iter = map.prefix(prefix).range(store, start, None, Order::Ascending);
    collect(iter, limit, parse_fn)
}

/// Iterate entries in a `cw_storage_plus::IndexedMap` under a given prefix.
///
/// TODO: add docs
pub fn paginate_indexed_map<'a, K, T, I, R, E, F>(
    map: &IndexedMap<K, T, I>,
    store: &dyn Storage,
    start: Option<Bound<'a, K>>,
    limit: Option<u32>,
    parse_fn: F,
) -> Result<Vec<R>, E>
where
    K: PrimaryKey<'a> + KeyDeserialize,
    K::Output: 'static,
    T: Serialize + DeserializeOwned + Clone,
    I: IndexList<T>,
    F: Fn(K::Output, T) -> Result<R, E>,
    E: From<StdError>,
{
    let iter = map.range(store, start, None, Order::Ascending);
    collect(iter, limit, parse_fn)
}

/// Iterate entries in a `cw_storage_plus::Map` and returns PaginatedResponse.
pub fn paginate_map_query<'a, K, T, R, E, F>(
    map: &Map<K, T>,
    store: &dyn Storage,
    start: Option<Bound<'a, K>>,
    limit: Option<u32>,
    map_fn: F,
) -> Result<PaginationResponse<R>, E>
where
    K: PrimaryKey<'a> + KeyDeserialize,
    K::Output: 'static,
    T: Serialize + DeserializeOwned,
    F: Fn(K::Output, T) -> Result<R, E>,
    E: From<StdError>,
{
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let limit_plus_one = Some((limit + 1) as u32);
    let mut data = paginate_map(map, store, start, limit_plus_one, map_fn)?;

    let has_more = data.len() > limit;
    if has_more {
        data.pop();
    }

    Ok(PaginationResponse {
        data,
        metadata: Metadata {
            has_more,
        },
    })
}

/// Iterate entries in a `cw_storage_plus::Map` under a given prefix and returns PaginatedResponse.
pub fn paginate_prefix_query<'a, K, T, R, E, F>(
    map: &Map<K, T>,
    store: &dyn Storage,
    prefix: K::Prefix,
    start: Option<Bound<'a, K::Suffix>>,
    limit: Option<u32>,
    map_fn: F,
) -> Result<PaginationResponse<R>, E>
where
    K: PrimaryKey<'a>,
    K::Suffix: PrimaryKey<'a> + KeyDeserialize,
    <K::Suffix as KeyDeserialize>::Output: 'static,
    T: Serialize + DeserializeOwned,
    F: Fn(<K::Suffix as KeyDeserialize>::Output, T) -> Result<R, E>,
    E: From<StdError>,
{
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let limit_plus_one = Some((limit + 1) as u32);
    let mut data = paginate_map_prefix(map, store, prefix, start, limit_plus_one, map_fn)?;

    let has_more = data.len() > limit;
    if has_more {
        data.pop();
    }

    Ok(PaginationResponse {
        data,
        metadata: Metadata {
            has_more,
        },
    })
}

// TODO: add unit tests
