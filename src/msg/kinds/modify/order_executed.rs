
use nom::number::streaming::{ be_u64, be_u32 };

/// Sent whenever an order on the book is executed in whole or in part. 
/// Messages are cumulative for orders executed in parts.
/// By combining these messages and the Trade Message types,
/// one can build a complete view of all executions on NASDAQ. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderExecuted {
    /// Identifier of the order for which a trade was executed.
    pub order_ref_num: u64,
    /// Quantity of shares traded.
    pub quantity: u32,
    /// Identifier for the trade event.
    pub match_number: u64,
}

impl OrderExecuted {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        let (input, quantity) = be_u32(input)?;
        let (input, match_number) = be_u64(input)?;

        Ok((input, Self {
            order_ref_num,
            quantity,
            match_number
        }))
    }
}

/// Sent whenever an order on the book is executed in whole or in part 
/// at a price different from the initial display price.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderExecutedWithPrice {
    /// Identifier of the order for which a trade was executed.
    pub order_ref_num: u64,
    /// Quantity of shares traded.
    pub quantity: u32,
    /// Identifier for the trade event.
    pub match_number: u64,
    // pub printable: bool,
    // pub price: Price32
}

impl OrderExecutedWithPrice {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        let (input, quantity) = be_u32(input)?;
        let (input, match_number) = be_u64(input)?;

        Ok((input, Self {
            order_ref_num,
            quantity,
            match_number,
        }))
    }
}

