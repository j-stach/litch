
use nom::number::streaming::{ be_u64, be_u32 };

/// Sent whenever an order is modified as a result of a partial cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderCancel {
    /// Identifier for the canceled order.
    pub order_ref_num: u64,
    /// Number of shares being removed from the order's display size.
    pub quantity: u32,
}

impl OrderCancel {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        let (input, quantity) = be_u32(input)?;

        Ok((input, Self {
            order_ref_num,
            quantity,
        }))
    }
}

/// Sent whenever an order on the book is being cancelled. 
/// Shares are no longer accessible and the order must be removed from the book.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderDelete {
    /// Identifier for the deleted order.
    pub order_ref_num: u64,
}

impl OrderDelete {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        Ok((input, Self { order_ref_num }))
    }
}

