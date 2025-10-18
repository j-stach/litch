
/// Regulation SHO Short Sale Price Test Restriction status for the issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegShoAction {

    /// No price test in place.
    NoPriceTest,

    /// Reg SHO Short Sale Price Test Restriction in effect due to
    /// an intra-day price drop in security.
    PriceDrop,

    /// Reg SHO Short Sale Price Test Restriction remains in effect.
    RemainInEffect,
}

use nom::{ Parser, branch::alt, combinator::map, character::char };

impl RegShoAction {
    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, action) = alt((
            map(char('0'), |_| Self::NoPriceTest),
            map(char('1'), |_| Self::PriceDrop),
            map(char('2'), |_| Self::RemainInEffect),
        )).parse(input)?;

        Ok((input, action))
    }
}
