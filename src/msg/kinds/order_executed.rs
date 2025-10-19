
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderExecuted {
    // pub order_ref_num: u64,
    // pub quantity: u32,
    // pub match_number: u64,
}

impl OrderExecuted {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /*stock,*/ }))
    }

}

/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderExecutedWithPrice {
    // pub order_ref_num: u64,
    // pub quantity: u32,
    // pub match_number: u64,
    // pub printable: bool,
    // pub price: Price32
}

impl OrderExecutedWithPrice {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /*stock,*/ }))
    }

}

