use std::collections::{HashMap,HashSet};

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
    pub reverse: bool,
}

impl Member {
    pub fn new(id: u64, role: MemberRole, member_type: MemberType) -> Self {
        Member { id, role, member_type, reverse: false }
    }
    pub fn drain(members: &mut Vec<Member>, ways: &HashMap<i64, Vec<i64>>) -> () {
        // the only members that matter for rendering purposes are inner and outer ways
        members.drain_filter(|m| {
            let ref_len = ways.get(&(m.id as i64)).and_then(|refs| Some(refs.len()));
            match (&m.role,&m.member_type,ref_len) {
                (MemberRole::Inner(),MemberType::Way(),Some(len)) => len > 0,
                (MemberRole::Outer(),MemberType::Way(),Some(len)) => len > 0,
                _ => false,
            }
        });
    }
    pub fn sort(members: &[Member], ways: &HashMap<i64, Vec<i64>>) -> Vec<Member> {
        if members.is_empty() { return vec![] }
        let mut first_ids: HashMap<i64,usize> = HashMap::new();
        let mut last_ids: HashMap<i64,usize> = HashMap::new();
        for (i,m) in members.iter().enumerate() {
            let refs = ways.get(&(m.id as i64)).unwrap();
            first_ids.insert(*refs.first().unwrap(), i);
            last_ids.insert(*refs.last().unwrap(), i);
        }
        let mut i = 0;
        let mut j = 0;
        let mut visited = HashSet::new();
        let mut sorted = vec![];
        let mut reverse = false;
        while i < members.len() {
            if visited.contains(&i) {
                i = j;
                j += 1;
                continue;
            }
            visited.insert(i);
            let mut m = members[i].clone();
            m.reverse = reverse;
            let id = m.id as i64;
            sorted.push(m);
            if !ways.contains_key(&id) {
                i = j;
                j += 1;
                continue;
            }
            let refs = ways.get(&id).unwrap();
            let first = refs.first().unwrap();
            let last = refs.last().unwrap();
            let fif = first_ids.get(first).unwrap();
            let lif = last_ids.get(first).unwrap();
            let fil = first_ids.get(last).unwrap();
            let lil = last_ids.get(last).unwrap();
            if !visited.contains(&fif) {
                i = *fif;
                sorted.last_mut().unwrap().reverse = true;
                reverse = false;
            } else if !visited.contains(&lif) {
                i = *lif;
                sorted.last_mut().unwrap().reverse = true;
                reverse = true;
            } else if !visited.contains(&fil) {
                i = *fil;
                sorted.last_mut().unwrap().reverse = false;
                reverse = false;
            } else if !visited.contains(&lil) {
                i = *lil;
                sorted.last_mut().unwrap().reverse = false;
                reverse = true;
            } else {
                i = j;
                j += 1;
            }
        }
        sorted
    }
}
