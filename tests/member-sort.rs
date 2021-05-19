use georender_pack::{Member, MemberRole, MemberType};
use std::collections::HashMap;
use pretty_assertions::assert_eq;

type Error = Box<dyn std::error::Error+Send+Sync>;

#[test] fn inner_before_outer() -> Result<(),Error> {
    let members = vec![
        Member {
            id: 477502096,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 477512773,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
    ];
    let mut ways = HashMap::new();
    ways.insert(477512773, vec![4708510052, 4708510053, 4708510054, 4708510055, 4708510056, 4708510052]);
    ways.insert(477502096, vec![4708432457, 4708432458, 4708432459, 4708432460, 4708432457]);
    assert_eq![Member::sort(&members, &ways), vec![
        Member {
            id: 477512773,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 477502096,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
    ]];
    Ok(())
}

#[test] fn inner_outer_inner() -> Result<(),Error> {
    let members = vec![
        Member {
            id: 1,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 2,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 3,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
    ];
    let mut ways = HashMap::new();
    ways.insert(1, vec![10,11]);
    ways.insert(2, vec![20,21,22,23,20]);
    ways.insert(3, vec![11,12,10]);
    assert_eq![Member::sort(&members, &ways), vec![
        Member {
            id: 2,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 1,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 3,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
    ]];
    Ok(())
}

#[test] fn inner_outer3_inner() -> Result<(),Error> {
    let members = vec![
        Member {
            id: 1,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 2,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 3,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 4,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 5,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
    ];
    let mut ways = HashMap::new();
    ways.insert(1, vec![10,11]);
    ways.insert(2, vec![20,21]);
    ways.insert(3, vec![21,22]);
    ways.insert(4, vec![22,23,20]);
    ways.insert(5, vec![11,12,10]);
    assert_eq![Member::sort(&members, &ways), vec![
        Member {
            id: 2,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 3,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 4,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 1,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 5,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
    ]];
    Ok(())
}

#[test] fn inner2_outer3_inner() -> Result<(),Error> {
    let members = vec![
        Member {
            id: 1,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 2,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 3,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 4,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 5,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 6,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
    ];
    let mut ways = HashMap::new();
    ways.insert(1, vec![10,11]);
    ways.insert(2, vec![11,12]);
    ways.insert(3, vec![20,21]);
    ways.insert(4, vec![21,22]);
    ways.insert(5, vec![22,23,20]);
    ways.insert(6, vec![12,10]);
    assert_eq![Member::sort(&members, &ways), vec![
        Member {
            id: 3,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 4,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 5,
            role: MemberRole::Outer(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 1,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 2,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
        Member {
            id: 6,
            role: MemberRole::Inner(),
            member_type: MemberType::Way(),
            reverse: false
        },
    ]];
    Ok(())
}
