
use nom::number::streaming::{ be_u64, be_u32 };
use nsdq_util::{ 
    define_enum, 
    StockSymbol, 
    Price,
    Mpid,
    parse_bool,
};

/// Generated for new orders accepted by NASDAQ. 
/// Represents Type "A" messages (i.e., without explicit MPID attribution).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderAdded {

    /// Day-unique identifier used to track the order.
    pub order_ref_num: u64,
    /// Buy/Sell indicator.
    pub side: Side,
    /// Number of shares for the order.
    pub quantity: u32,
    /// Stock symbol for which the order was placed.
    pub stock: StockSymbol,
    /// Price for which the order was placed.
    pub price: Price<u32, 4>,
}

impl OrderAdded {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        let (input, side) = Side::parse(input)?;
        let (input, quantity) = be_u32(input)?;
        let (input, stock) = StockSymbol::parse(input)?;
        let (input, price) = Price::<u32, 4>::parse(input)?;

        Ok((input, Self { 
            order_ref_num,
            side,
            quantity,
            stock,
            price,
        }))
    }
}


/// Generated for new orders accepted by NASDAQ. 
/// Represents Type "F" messages (i.e., with explicit MPID attribution).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderAddedWithMpid {

    /// Day-unique identifier used to track the order.
    pub order_ref_num: u64,
    /// Buy/Sell indicator.
    pub side: Side,
    /// Number of shares for the order.
    pub quantity: u32,
    /// Stock symbol for which the order was placed.
    pub stock: StockSymbol,
    /// Price for which the order was placed.
    pub price: Price<u32, 4>,
    /// Market Participant ID (MPID) attribution for the order.
    /// NOTE: Used only for Type "F" OrderAdded messages (section 1.3.2).
    /// NOTE: If a firm wants to display a MPID for unattributed orders, 
    /// Nasdaq recommends that it use the MPID of “NSDQ”.
    pub mpid: Mpid,
}

impl OrderAddedWithMpid {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        let (input, side) = Side::parse(input)?;
        let (input, quantity) = be_u32(input)?;
        let (input, stock) = StockSymbol::parse(input)?;
        let (input, price) = Price::<u32, 4>::parse(input)?;
        let (input, mpid) = Mpid::parse(input)?;

        Ok((input, Self { 
            order_ref_num,
            side,
            quantity,
            stock,
            price,
            mpid
        }))
    }

}

define_enum!{

    Side:
        "Buy/Sell Indicator";

    ['B'] Buy
        "Buy",
    ['S'] Sell
        "Sell",
}


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
    /// If true, execution will be reflected on time and sales displays 
    /// and volume calculations.
    pub printable: bool,
    /// Price at which the trade executed.
    /// Value will likely be different from what it was in `OrderAdded`.
    pub price: Price<u32, 4>,
}

impl OrderExecutedWithPrice {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        let (input, quantity) = be_u32(input)?;
        let (input, match_number) = be_u64(input)?;
        let (input, printable) = parse_bool(input)?;
        let (input, price) = Price::<u32, 4>::parse(input)?;

        Ok((input, Self {
            order_ref_num,
            quantity,
            match_number,
            printable,
            price,
        }))
    }
}


/// Sent whenever an order is modified as a result of a partial cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderCanceled {

    /// Identifier for the canceled order.
    pub order_ref_num: u64,
    /// Number of shares being removed from the order's display size.
    pub quantity: u32,
}

impl OrderCanceled {

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
pub struct OrderDeleted {

    /// Identifier for the deleted order.
    pub order_ref_num: u64,
}

impl OrderDeleted {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        Ok((input, Self { order_ref_num }))
    }
}


/// Sent whenever an order on the book has been cancel-replaced. 
/// All remaining shares from the original order are no longer accessible, 
/// and must be removed. 
///
/// The new order details are provided for the replacement, 
/// along with a new order reference number. 
/// Since the side, stock symbol and attribution cannot be changed by a Replace,
/// firms should retain these values from the original order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderReplaced {

    /// Identifier for the canceled order.
    pub old_ref_num: u64,
    /// Identifier for the replacement order.
    pub new_ref_num: u64,
    /// New quantity of shares.
    pub quantity: u32,
    /// Replacement price.
    pub price: Price<u32, 4>,
}

impl OrderReplaced {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, old_ref_num) = be_u64(input)?;
        let (input, new_ref_num) = be_u64(input)?;
        let (input, quantity) = be_u32(input)?;
        let (input, price) = Price::<u32, 4>::parse(input)?;

        Ok((input, Self {
            old_ref_num,
            new_ref_num,
            quantity,
            price,
        }))
    }

}

