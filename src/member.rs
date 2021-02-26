#[derive(Clone)]
pub enum MemberRole {
    Inner(),
    Outer(),
    Unused(),
}

#[derive(Clone)]
pub enum MemberType {
    Node(),
    Way(),
    Relation(),
}

#[derive(Clone)]
pub struct Member {
    pub id: u64,
    pub role: MemberRole,
    pub member_type: MemberType,
}

impl Member {
    pub fn new(id: u64, role: MemberRole, member_type: MemberType) -> Self {
        Member { id, role, member_type }
    }
    pub fn drop(members: &mut Vec<Member>) -> () {
        // todo
    }
    pub fn sort(members: &mut Vec<Member>) -> () {
        // todo
    }
}
