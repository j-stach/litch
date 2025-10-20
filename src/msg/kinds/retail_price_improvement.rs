
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetailPriceImprovement {
    // stock
    pub interest_flag: InterestFlag
}

impl RetailPriceImprovement {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        let (input, interest_flag) = InterestFlag::parse(input)?;

        Ok((input, Self { interest_flag }))
    }

}

crate::types::define_enum!{

    InterestFlag:
        "Availability of Retail Price Improvement orders.";

    ['B'] BuyAvailable
        "RPI orders available on the buy side.",
    ['S'] SellAvailable
        "RPI orders available on the sell side.",
    ['A'] AnyAvailable
        "RPI orders available on both sides (buy and sell).",
    ['N'] NoneAvailable
        "No RPI orders available.",
}

