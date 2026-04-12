#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ApoptosisLevel {
    None,
    LowEnergy,
    Starving,
    Critical,
}
