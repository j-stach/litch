
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderCancel {
    // pub order_ref_num: u64,
    // pub quantity: u32,
}

impl OrderCancel {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /*stock,*/ }))
    }

}

/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderDelete {
    // pub order_ref_num: u64,
}

impl OrderDelete {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /*stock,*/ }))
    }

}

