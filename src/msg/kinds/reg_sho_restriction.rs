
use crate::types::{ RegShoAction, StockSymbol };

/// Reg SHO Short Sale Price Test Restricted Indicator.
/// Pre-opening spin indicates the Rule 201 status for all active issues. 
/// Nasdaq also sends this message in the event of an intraday status change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegShoRestriction {
    //TODO: Convert slouch to use nom for parsing responses
    //pub stock: StockSymbol,
    pub action: RegShoAction,
}

impl RegShoRestriction {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        let (input, action) = RegShoAction::parse(input)?;

        Ok((input, Self { /*stock,*/ action }))
    }

}
