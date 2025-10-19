
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MwcbStatus {
    /// Denotes the MWCB Level that was breached.
    pub level: BreachedLevel
}

impl MwcbStatus {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        let (input, level) = BreachedLevel::parse(input)?;

        Ok((input, Self { level }))
    }

}

crate::types::define_enum!{

    BreachedLevel:
        "";

    ['1'] Level1
        "",
    ['2'] Level2
        "",
    ['3'] Level3
        "",
}

