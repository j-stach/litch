
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddOrder {
    // pub order_ref_num: u64,
    pub side: Side,
    // pub quantity: u32,
    // TODO pub stock: StockSymbol,
    // pub price: Price32
}

impl AddOrder {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        let (input, side) = Side::parse(input)?;
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /*stock,*/ side }))
    }

}

/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddOrderWithAttr {
    // pub order_ref_num: u64,
    pub side: Side,
    // pub quantity: u32,
    // TODO pub stock: StockSymbol,
    // pub price: Price32,
    // pub attribution: str4
}

impl AddOrderWithAttr {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        let (input, side) = Side::parse(input)?;
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /*stock,*/ side }))
    }

}


crate::types::define_enum!{

    Side:
        "";

    ['B'] Buy
        "",
    ['S'] Sell
        "",
}

