use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum MadhabLocal {
    Shafi = 1,
    Hanafi = 2,
}

impl From<MadhabLocal> for salah::Madhab {
    fn from(madhab: MadhabLocal) -> salah::Madhab {
        match madhab {
            MadhabLocal::Shafi => salah::Madhab::Shafi,
            MadhabLocal::Hanafi => salah::Madhab::Hanafi,
        }
    }
}
