
use nom::number::streaming::be_u32;

/// Auction collar thresholds within which a paused security can reopen 
/// following a Limit-Up/Limit-down (LULD) Trading Pause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LuldAuctionCollar {
    // TODO pub stock: StockSymbol,
    // TODO pub reference_price: Price32,
    // TODO pub upper_price: Price32,
    // TODO pub lower_price: Price32,
    /// Indicates the number of the extensions to the Reopening Auction.
    pub extension: u32,
}

impl LuldAuctionCollar {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;
        let (input, extension) = be_u32(input)?;

        Ok((input, Self {
            // stock,
            // reference_price,
            // upper_price,
            // lower_price,
            extension
        }))
    }
}

