use cosmwasm_std::{testing::mock_dependencies, StdError, Uint128};
use cw_paginate::paginate_prefix_query;
use cw_storage_plus::{Bound, Map};

#[test]
fn empty_when_prefix_not_found() {
    let mut deps = mock_dependencies();

    let map: Map<(&str, &str), Uint128> = Map::new("map");

    map.save(&mut deps.storage, ("prefix_1", "key_1"), &Uint128::new(1)).unwrap();
    map.save(&mut deps.storage, ("prefix_1", "key_2"), &Uint128::new(2)).unwrap();
    map.save(&mut deps.storage, ("prefix_2", "key_1"), &Uint128::new(3)).unwrap();
    map.save(&mut deps.storage, ("prefix_3", "key_1"), &Uint128::new(4)).unwrap();

    let res =
        paginate_prefix_query(&map, &deps.storage, "prefix_x", None, Some(3), |_key, amount| {
            Ok::<Uint128, StdError>(amount)
        })
        .unwrap();

    assert!(!res.metadata.has_more);
    assert_eq!(res.data.len(), 0);
}

#[test]
fn has_more_false_when_all_prefixes_within_limit() {
    let mut deps = mock_dependencies();

    let map: Map<(&str, &str), Uint128> = Map::new("map");

    map.save(&mut deps.storage, ("prefix_1", "key_1"), &Uint128::new(1)).unwrap();
    map.save(&mut deps.storage, ("prefix_1", "key_2"), &Uint128::new(2)).unwrap();
    map.save(&mut deps.storage, ("prefix_2", "key_1"), &Uint128::new(3)).unwrap();
    map.save(&mut deps.storage, ("prefix_3", "key_1"), &Uint128::new(4)).unwrap();

    let res =
        paginate_prefix_query(&map, &deps.storage, "prefix_1", None, Some(3), |_key, amount| {
            Ok::<Uint128, StdError>(amount)
        })
        .unwrap();

    assert_eq!(res.data.get(0).unwrap(), Uint128::new(1));
    assert_eq!(res.data.get(1).unwrap(), Uint128::new(2));
    assert!(!res.metadata.has_more);
    assert_eq!(res.data.len(), 2);
}

#[test]
fn has_more_true_when_results_outside_limit() {
    let mut deps = mock_dependencies();

    let map: Map<(&str, &str), Uint128> = Map::new("map");

    map.save(&mut deps.storage, ("prefix_1", "key_1"), &Uint128::new(1)).unwrap();
    map.save(&mut deps.storage, ("prefix_1", "key_2"), &Uint128::new(2)).unwrap();
    map.save(&mut deps.storage, ("prefix_2", "key_1"), &Uint128::new(3)).unwrap();
    map.save(&mut deps.storage, ("prefix_3", "key_1"), &Uint128::new(4)).unwrap();

    let res =
        paginate_prefix_query(&map, &deps.storage, "prefix_1", None, Some(1), |_key, amount| {
            Ok::<Uint128, StdError>(amount)
        })
        .unwrap();

    assert_eq!(res.data.get(0).unwrap(), Uint128::new(1));
    assert!(res.metadata.has_more);
    assert_eq!(res.data.len(), 1);
}

#[test]
fn empty_when_start_after_not_found() {
    let mut deps = mock_dependencies();

    let map: Map<(&str, &str), Uint128> = Map::new("map");

    map.save(&mut deps.storage, ("prefix_1", "key_1"), &Uint128::new(1)).unwrap();
    map.save(&mut deps.storage, ("prefix_1", "key_2"), &Uint128::new(2)).unwrap();
    map.save(&mut deps.storage, ("prefix_2", "key_1"), &Uint128::new(3)).unwrap();
    map.save(&mut deps.storage, ("prefix_3", "key_1"), &Uint128::new(4)).unwrap();

    let res = paginate_prefix_query(
        &map,
        &deps.storage,
        "prefix_1",
        Some(Bound::inclusive("key_x")),
        Some(1),
        |_key, amount| Ok::<Uint128, StdError>(amount),
    )
    .unwrap();

    assert!(!res.metadata.has_more);
    assert_eq!(res.data.len(), 0);
}

#[test]
fn has_more_false_when_start_is_last_alphabetically() {
    let mut deps = mock_dependencies();

    let map: Map<(&str, &str), Uint128> = Map::new("map");

    map.save(&mut deps.storage, ("prefix_1", "key_1"), &Uint128::new(1)).unwrap();
    map.save(&mut deps.storage, ("prefix_1", "key_2"), &Uint128::new(2)).unwrap();
    map.save(&mut deps.storage, ("prefix_2", "key_1"), &Uint128::new(3)).unwrap();
    map.save(&mut deps.storage, ("prefix_3", "key_1"), &Uint128::new(4)).unwrap();

    let res = paginate_prefix_query(
        &map,
        &deps.storage,
        "prefix_1",
        Some(Bound::inclusive("key_2")),
        Some(1),
        |_key, amount| Ok::<Uint128, StdError>(amount),
    )
    .unwrap();

    assert_eq!(res.data.get(0).unwrap(), Uint128::new(2));
    assert!(!res.metadata.has_more);
    assert_eq!(res.data.len(), 1);
}
