
// TODO: May be able to combine these types

use nom::number::streaming::{ be_u64, be_u32 };

/// Generated for unattributed orders accepted by NASDAQ. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddOrder {
    pub order_ref_num: u64,
    pub side: Side,
    pub quantity: u32,
    // TODO pub stock: StockSymbol,
    // pub price: Price32
}

impl AddOrder {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        let (input, order_ref_num) = be_u64(input)?;
        let (input, side) = Side::parse(input)?;
        let (input, quantity) = be_u32(input)?;
        // let (input, stock) = StockSymbol::parse(input)?;
        // let (input, price) = be_u32(input)?;

        Ok((input, Self { 
            order_ref_num,
            side,
            quantity,
            // stock,
            // price
        }))
    }

}

/// Generated for attributed orders and quotations accepted by NASDAQ.
/// (NOTE: If a firm wants to display a MPID for unattributed orders, 
/// Nasdaq recommends that it use the MPID of “NSDQ”.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddOrderWithAttr {
    pub order_ref_num: u64,
    pub side: Side,
    pub quantity: u32,
    // TODO pub stock: StockSymbol,
    // pub price: Price32,
    // pub attribution: str4
}

impl AddOrderWithAttr {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        let (input, order_ref_num) = be_u64(input)?;
        let (input, side) = Side::parse(input)?;
        let (input, quantity) = be_u32(input)?;
        // let (input, stock) = StockSymbol::parse(input)?;
        // let (input, price) = be_u32(input)?;
        // let (input, attribution) = Attribution::parse(input)?;

        Ok((input, Self { 
            order_ref_num,
            side,
            quantity,
            // stock,
            // price,
            // attribution
        }))
    }

}

crate::types::define_enum!{

    Side:
        "Buy/Sell Indicator";

    ['B'] Buy
        "Buy",
    ['S'] Sell
        "Sell",
}

// TODO: May be able to combine these types

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

