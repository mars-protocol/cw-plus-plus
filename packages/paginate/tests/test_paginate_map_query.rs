use cosmwasm_std::{testing::mock_dependencies, StdError, Uint128};
use cw_paginate::paginate_map_query;
use cw_storage_plus::{Bound, Map};

#[test]
fn empty_when_start_not_found() {
    let mut deps = mock_dependencies();

    let map: Map<&str, Uint128> = Map::new("map");

    map.save(&mut deps.storage, "key_1", &Uint128::new(1)).unwrap();
    map.save(&mut deps.storage, "key_2", &Uint128::new(2)).unwrap();
    map.save(&mut deps.storage, "key_3", &Uint128::new(3)).unwrap();

    let res = paginate_map_query(
        &map,
        &deps.storage,
        Some(Bound::exclusive("key_x")),
        Some(3),
        |_key, amount| Ok::<Uint128, StdError>(amount),
    )
    .unwrap();

    assert!(!res.metadata.has_more);
    assert_eq!(res.data.len(), 0);
}

#[test]
fn has_more_true_when_limit_not_reached() {
    let mut deps = mock_dependencies();

    let map: Map<&str, Uint128> = Map::new("map");

    map.save(&mut deps.storage, "key_1", &Uint128::new(1)).unwrap();
    map.save(&mut deps.storage, "key_2", &Uint128::new(2)).unwrap();
    map.save(&mut deps.storage, "key_3", &Uint128::new(3)).unwrap();

    let res = paginate_map_query(&map, &deps.storage, None, Some(2), |_key, amount| {
        Ok::<Uint128, StdError>(amount)
    })
    .unwrap();

    assert_eq!(res.data.get(0).unwrap(), Uint128::new(1));
    assert_eq!(res.data.get(1).unwrap(), Uint128::new(2));
    assert!(res.metadata.has_more);
    assert_eq!(res.data.len(), 2);
}

#[test]
fn has_more_false_when_limit_reached() {
    let mut deps = mock_dependencies();

    let map: Map<&str, Uint128> = Map::new("map");

    map.save(&mut deps.storage, "key_1", &Uint128::new(1)).unwrap();
    map.save(&mut deps.storage, "key_2", &Uint128::new(2)).unwrap();
    map.save(&mut deps.storage, "key_3", &Uint128::new(3)).unwrap();

    let res = paginate_map_query(&map, &deps.storage, None, Some(3), |_key, amount| {
        Ok::<Uint128, StdError>(amount)
    })
    .unwrap();

    assert_eq!(res.data.get(0).unwrap(), Uint128::new(1));
    assert_eq!(res.data.get(1).unwrap(), Uint128::new(2));
    assert_eq!(res.data.get(2).unwrap(), Uint128::new(3));
    assert!(!res.metadata.has_more);
    assert_eq!(res.data.len(), 3);
}

#[test]
fn empty_when_map_is_empty() {
    let deps = mock_dependencies();

    let map: Map<&str, Uint128> = Map::new("map");

    let res = paginate_map_query(&map, &deps.storage, None, Some(3), |_key, amount| {
        Ok::<Uint128, StdError>(amount)
    })
    .unwrap();

    assert!(!res.metadata.has_more);
    assert_eq!(res.data.len(), 0);
}

#[test]
fn has_more_false_when_start_is_last_alphabetically() {
    let mut deps = mock_dependencies();

    let map: Map<&str, Uint128> = Map::new("map");

    map.save(&mut deps.storage, "key_3", &Uint128::new(3)).unwrap();
    map.save(&mut deps.storage, "key_2", &Uint128::new(2)).unwrap();
    map.save(&mut deps.storage, "key_1", &Uint128::new(1)).unwrap();

    let res = paginate_map_query(
        &map,
        &deps.storage,
        Some(Bound::inclusive("key_3")),
        Some(3),
        |_key, amount| Ok::<Uint128, StdError>(amount),
    )
    .unwrap();

    assert_eq!(res.data.get(0).unwrap(), Uint128::new(3));
    assert!(!res.metadata.has_more);
    assert_eq!(res.data.len(), 1);
}
