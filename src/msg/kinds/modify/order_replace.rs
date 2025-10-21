
use nom::number::streaming::{ be_u64, be_u32 };

/// Sent whenever an order on the book has been cancel-replaced. 
/// All remaining shares from the original order are no longer accessible, 
/// and must be removed. 
///
/// The new order details are provided for the replacement, 
/// along with a new order reference number. 
/// Since the side, stock symbol and attribution cannot be changed by a Replace,
/// firms should retain these values from the original order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderReplace {

    /// Identifier for the canceled order.
    pub old_ref_num: u64,
    /// Identifier for the replacement order.
    pub new_ref_num: u64,
    /// New quantity of shares.
    pub quantity: u32,
    // pub price: Price32
}

impl OrderReplace {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, old_ref_num) = be_u64(input)?;
        let (input, new_ref_num) = be_u64(input)?;
        let (input, quantity) = be_u32(input)?;

        Ok((input, Self {
            old_ref_num,
            new_ref_num,
            quantity
        }))
    }

}

