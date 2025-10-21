
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

