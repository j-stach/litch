
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MwcbDeclineLevel {
    // TODO pub level_1: Price64
    // TODO pub level_2: Price64
    // TODO pub level_3: Price64
}

impl MwcbDeclineLevel {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /**/ }))
    }

}
