/// Describes how much detail should be outputted
#[non_exhaustive]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum LevelOfDetail {
    /// A standard and informative level of information.
    Standard,
    /// Provide as much detail as available
    Concise,
    /// Provide as much information as possible
    Verbose,
    /// Do not output anything
    None
}
