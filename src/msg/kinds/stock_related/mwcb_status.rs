
/// Sent when a Market-wide Circuit Breaker (MWCB) has breached a level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MwcbStatus {
    /// Denotes the MWCB Level that was breached.
    pub level: BreachedLevel
}

impl MwcbStatus {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, level) = BreachedLevel::parse(input)?;
        Ok((input, Self { level }))
    }
}

crate::types::define_enum!{

    BreachedLevel:
        "Denotes the MWCB Level that was breached.";

    ['1'] _1
        "Level 1",
    ['2'] _2
        "Level 2",
    ['3'] _3
        "Level 3",
}

