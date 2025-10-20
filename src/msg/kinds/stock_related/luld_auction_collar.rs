
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LuldAuctionCollar {
    // TODO pub stock: StockSymbol,
    // TODO pub reference_price: Price32,
    // TODO pub upper_price: Price32,
    // TODO pub lower_price: Price32,
    // TODO pub extension: u32,

}

impl LuldAuctionCollar {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /*stock,*/ }))
    }

}

