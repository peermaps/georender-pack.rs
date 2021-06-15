use georender_pack::{
    decode, Feature, Area,
    Member, MemberRole, MemberType,
    osm_types::get_types,
    encode::{relation, relation_from_parsed}};
use std::collections::HashMap;
use pretty_assertions::assert_eq;

type Error = Box<dyn std::error::Error+Send+Sync>;

#[test] fn relation_area_from_parsed_0() -> Result<(),Error> {
    let tags = vec![("type","multipolygon"),("natural", "water")];
    let mut nodes = HashMap::new();
    nodes.insert(100, (1.3, 1.2));
    nodes.insert(101, (1.3, 0.3));
    nodes.insert(102, (-0.1, 0.3));
    nodes.insert(103, (-0.1, 1.2));
    nodes.insert(104, (0.8, 0.7));
    nodes.insert(105, (0.5, 0.5));
    nodes.insert(106, (1.0, 0.6));
    let mut ways = HashMap::new();
    ways.insert(200, vec![100,101,102,103,100]);
    ways.insert(202, vec![104,105,106,104]);

    let positions = vec![
        1.3, 1.2, 1.3, 0.3, -0.1, 0.3, -0.1, 1.2,
        0.8, 0.7, 0.5, 0.5, 1.0, 0.6
    ];
    let cells = earcutr::earcut(&positions, &vec![4], 2);
    let feature_type = *get_types().get("natural.water").unwrap();
    let expected = Feature::Area(Area {
        id: 1234,
        feature_type,
        labels: vec![0],
        positions: positions.iter().map(|p| *p as f32).collect(),
        cells,
    });
    let members = vec![
        Member::new(200, MemberRole::Outer(), MemberType::Way()),
        Member::new(202, MemberRole::Inner(), MemberType::Way()),
    ];
    assert_eq![&expected, &decode(&relation(1234, &tags, &members, &nodes, &ways)?)?];
    assert_eq![&expected, &decode(
        &relation_from_parsed(1234, feature_type, true, &vec![0], &members, &nodes, &ways)?
    )?];
    Ok(())
}

#[test] fn relation_area_from_parsed_1() -> Result<(),Error> {
    let tags = vec![("type","multipolygon"),("natural", "water")];
    let mut nodes = HashMap::new();
    nodes.insert(100, (1.3, 1.2));
    nodes.insert(101, (1.3, 0.3));
    nodes.insert(102, (-0.1, 0.3));
    nodes.insert(103, (-0.1, 1.2));
    nodes.insert(104, (0.8, 0.7));
    nodes.insert(105, (0.5, 0.5));
    nodes.insert(106, (1.0, 0.6));
    let mut ways = HashMap::new();
    ways.insert(200, vec![100,101]);
    ways.insert(201, vec![101,102,103,100]);
    ways.insert(202, vec![104,105,106,104]);

    let positions = vec![
        1.3, 1.2, 1.3, 0.3, -0.1, 0.3, -0.1, 1.2,
        0.8, 0.7, 0.5, 0.5, 1.0, 0.6
    ];
    let cells = earcutr::earcut(&positions, &vec![4], 2);
    let feature_type = *get_types().get("natural.water").unwrap();
    let expected = Feature::Area(Area {
        id: 1234,
        feature_type,
        labels: vec![0],
        positions: positions.iter().map(|p| *p as f32).collect(),
        cells,
    });
    let members = vec![
        Member::new(200, MemberRole::Outer(), MemberType::Way()),
        Member::new(201, MemberRole::Outer(), MemberType::Way()),
        Member::new(202, MemberRole::Inner(), MemberType::Way()),
    ];
    assert_eq![&expected, &decode(&relation(1234, &tags, &members, &nodes, &ways)?)?];
    assert_eq![&expected, &decode(
        &relation_from_parsed(1234, feature_type, true, &vec![0], &members, &nodes, &ways)?
    )?];
    Ok(())
}

#[test] fn relation_area_from_parsed_2() -> Result<(),Error> {
    let tags = vec![("type","multipolygon"),("natural", "water")];
    let mut nodes = HashMap::new();
    nodes.insert(100, (1.3, 1.2));
    nodes.insert(101, (1.3, 0.3));
    nodes.insert(102, (-0.1, 0.3));
    nodes.insert(103, (-0.1, 1.2));
    nodes.insert(104, (0.8, 0.7));
    nodes.insert(105, (0.5, 0.5));
    nodes.insert(106, (1.0, 0.6));
    let mut ways = HashMap::new();
    ways.insert(200, vec![100,101]);
    ways.insert(201, vec![101,102,103,100]);
    ways.insert(202, vec![104,105]);
    ways.insert(203, vec![105,106,104]);

    let positions = vec![
        1.3, 1.2, 1.3, 0.3, -0.1, 0.3, -0.1, 1.2,
        0.8, 0.7, 0.5, 0.5, 1.0, 0.6
    ];
    let cells = earcutr::earcut(&positions, &vec![4], 2);
    let feature_type = *get_types().get("natural.water").unwrap();
    let expected = Feature::Area(Area {
        id: 1234,
        feature_type,
        labels: vec![0],
        positions: positions.iter().map(|p| *p as f32).collect(),
        cells,
    });
    let members = vec![
        Member::new(200, MemberRole::Outer(), MemberType::Way()),
        Member::new(201, MemberRole::Outer(), MemberType::Way()),
        Member::new(202, MemberRole::Inner(), MemberType::Way()),
        Member::new(203, MemberRole::Inner(), MemberType::Way()),
    ];
    assert_eq![&expected, &decode(&relation(1234, &tags, &members, &nodes, &ways)?)?];
    assert_eq![&expected, &decode(
        &relation_from_parsed(1234, feature_type, true, &vec![0], &members, &nodes, &ways)?
    )?];
    Ok(())
}

#[test] fn relation_area_from_parsed_3() -> Result<(),Error> {
    let tags = vec![("type","multipolygon"),("natural", "water")];
    let mut nodes = HashMap::new();
    nodes.insert(100, (1.3, 1.2));
    nodes.insert(101, (1.3, 0.3));
    nodes.insert(102, (-0.1, 0.3));
    nodes.insert(103, (-0.1, 1.2));
    nodes.insert(104, (0.8, 0.7));
    nodes.insert(105, (0.5, 0.5));
    nodes.insert(106, (1.0, 0.6));
    let mut ways = HashMap::new();
    ways.insert(200, vec![100,101]);
    ways.insert(201, vec![101,102]);
    ways.insert(202, vec![102,103]);
    ways.insert(203, vec![103,100]);
    ways.insert(204, vec![104,105]);
    ways.insert(205, vec![105,106]);
    ways.insert(206, vec![106,104]);

    let positions = vec![
        1.3, 1.2, 1.3, 0.3, -0.1, 0.3, -0.1, 1.2,
        0.8, 0.7, 0.5, 0.5, 1.0, 0.6
    ];
    let cells = earcutr::earcut(&positions, &vec![4], 2);
    let feature_type = *get_types().get("natural.water").unwrap();
    let expected = Feature::Area(Area {
        id: 1234,
        feature_type,
        labels: vec![0],
        positions: positions.iter().map(|p| *p as f32).collect(),
        cells,
    });
    let members = vec![
        Member::new(200, MemberRole::Outer(), MemberType::Way()),
        Member::new(201, MemberRole::Outer(), MemberType::Way()),
        Member::new(202, MemberRole::Outer(), MemberType::Way()),
        Member::new(203, MemberRole::Outer(), MemberType::Way()),
        Member::new(204, MemberRole::Inner(), MemberType::Way()),
        Member::new(205, MemberRole::Inner(), MemberType::Way()),
        Member::new(206, MemberRole::Inner(), MemberType::Way()),
    ];
    assert_eq![&expected, &decode(&relation(1234, &tags, &members, &nodes, &ways)?)?];
    assert_eq![&expected, &decode(
        &relation_from_parsed(1234, feature_type, true, &vec![0], &members, &nodes, &ways)?
    )?];
    Ok(())
}

#[test] fn relation_area_from_parsed_out_of_order_1() -> Result<(),Error> {
    let tags = vec![("type","multipolygon"),("natural", "water")];
    let mut nodes = HashMap::new();
    nodes.insert(100, (1.3, 1.2));
    nodes.insert(101, (1.3, 0.3));
    nodes.insert(102, (-0.1, 0.3));
    nodes.insert(103, (-0.1, 1.2));
    nodes.insert(104, (0.8, 0.7));
    nodes.insert(105, (0.5, 0.5));
    nodes.insert(106, (1.0, 0.6));
    let mut ways = HashMap::new();
    ways.insert(200, vec![100,101]);
    ways.insert(201, vec![101,102]);
    ways.insert(202, vec![102,103]);
    ways.insert(203, vec![103,100]);
    ways.insert(204, vec![104,105]);
    ways.insert(205, vec![105,106]);
    ways.insert(206, vec![106,104]);

    let positions = vec![
        1.3, 0.3, -0.1, 0.3, -0.1, 1.2, 1.3, 1.2,
        0.5, 0.5, 1.0, 0.6, 0.8, 0.7,
    ];
    let cells = earcutr::earcut(&positions, &vec![4], 2);
    let feature_type = *get_types().get("natural.water").unwrap();
    let expected = Feature::Area(Area {
        id: 1234,
        feature_type,
        labels: vec![0],
        positions: positions.iter().map(|p| *p as f32).collect(),
        cells,
    });
    let members = vec![
        Member::new(201, MemberRole::Outer(), MemberType::Way()),
        Member::new(200, MemberRole::Outer(), MemberType::Way()),
        Member::new(203, MemberRole::Outer(), MemberType::Way()),
        Member::new(202, MemberRole::Outer(), MemberType::Way()),
        Member::new(205, MemberRole::Inner(), MemberType::Way()),
        Member::new(204, MemberRole::Inner(), MemberType::Way()),
        Member::new(206, MemberRole::Inner(), MemberType::Way()),
    ];
    assert_eq![&expected, &decode(&relation(1234, &tags, &members, &nodes, &ways)?)?];
    assert_eq![&expected, &decode(
        &relation_from_parsed(1234, feature_type, true, &vec![0], &members, &nodes, &ways)?
    )?];
    Ok(())
}

/*
#[test] fn relation_area_from_parsed_out_of_order_2() -> Result<(),Error> {
    let tags = vec![("type","multipolygon"),("natural", "water")];
    let mut nodes = HashMap::new();
    nodes.insert(100, (1.3, 1.2));
    nodes.insert(101, (1.3, 0.3));
    nodes.insert(102, (-0.1, 0.3));
    nodes.insert(103, (-0.1, 1.2));
    nodes.insert(104, (0.8, 0.7));
    nodes.insert(105, (0.5, 0.5));
    nodes.insert(106, (1.0, 0.6));
    let mut ways = HashMap::new();
    ways.insert(200, vec![100,101]);
    ways.insert(201, vec![102,101]);
    ways.insert(202, vec![102,103]);
    ways.insert(203, vec![100,103]);
    ways.insert(204, vec![104,105]);
    ways.insert(205, vec![106,105]);
    ways.insert(206, vec![104,106]);

    let positions = vec![
        -0.1, 0.3, 1.3, 0.3, 1.3, 1.2, -0.1, 1.2,
        0.8, 0.7, 1.0, 0.6, 0.5, 0.5,
    ];
    let cells = earcutr::earcut(&positions, &vec![4], 2);
    let feature_type = *get_types().get("natural.water").unwrap();
    let expected = Feature::Area(Area {
        id: 1234,
        feature_type,
        labels: vec![0],
        positions: positions.iter().map(|p| *p as f32).collect(),
        cells,
    });
    let members = vec![
        Member::new(201, MemberRole::Outer(), MemberType::Way()),
        Member::new(200, MemberRole::Outer(), MemberType::Way()),
        Member::new(203, MemberRole::Outer(), MemberType::Way()),
        Member::new(202, MemberRole::Outer(), MemberType::Way()),
        Member::new(205, MemberRole::Inner(), MemberType::Way()),
        Member::new(204, MemberRole::Inner(), MemberType::Way()),
        Member::new(206, MemberRole::Inner(), MemberType::Way()),
    ];
    assert_eq![&expected, &decode(&relation(1234, &tags, &members, &nodes, &ways)?)?];
    assert_eq![&expected, &decode(
        &relation_from_parsed(1234, feature_type, true, &vec![0], &members, &nodes, &ways)?
    )?];
    Ok(())
}
*/
