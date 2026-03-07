use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MethodLocal {
    MuslimWorldLeague,
    Egyptian,
    Karachi,
    UmmAlQura,
    Dubai,
    MoonsightingCommittee,
    NorthAmerica,
    Kuwait,
    Qatar,
    Singapore,
    Tehran,
    Turkey,
    Other,
}

impl From<MethodLocal> for salah::Method {
    fn from(method: MethodLocal) -> salah::Method {
        match method {
            MethodLocal::MuslimWorldLeague => salah::Method::MuslimWorldLeague,
            MethodLocal::Egyptian => salah::Method::Egyptian,
            MethodLocal::Karachi => salah::Method::Karachi,
            MethodLocal::UmmAlQura => salah::Method::UmmAlQura,
            MethodLocal::Dubai => salah::Method::Dubai,
            MethodLocal::MoonsightingCommittee => salah::Method::MoonsightingCommittee,
            MethodLocal::NorthAmerica => salah::Method::NorthAmerica,
            MethodLocal::Kuwait => salah::Method::Kuwait,
            MethodLocal::Qatar => salah::Method::Qatar,
            MethodLocal::Singapore => salah::Method::Singapore,
            MethodLocal::Tehran => salah::Method::Tehran,
            MethodLocal::Turkey => salah::Method::Turkey,
            MethodLocal::Other => salah::Method::Other,
        }
    }
}
