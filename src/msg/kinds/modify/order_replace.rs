
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderReplace {
    // pub order_ref_num: u64,
    // pub order_ref_num: u64,
    // pub quantity: u32,
    // pub price: Price32
}

impl OrderReplace {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /*stock*/ }))
    }

}

