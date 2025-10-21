
//! NOTE: It is possible to receive multiple Trade Messages for the same order 
//! if that order is executed in several parts. 
//! Trade Messages for the same order are cumulative.
//! Trade Messages should be included in NASDAQ time-and-sales displays 
//! as well as volume and other market statistics. 
//! Since Trade Messages do not affect the book, however, they may be ignored 
//! by firms just looking to track the NASDAQ execution system display.

use nom::number::streaming::{ be_u64, be_u32 };

/// Provides execution details for normal matches of non-displayable orders. 
/// (Since no `AddOrder` is generated when a non-displayed order is received, 
/// NASDAQ cannot use the `OrderExecuted` messages for all trades.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchTrade {
    pub order_ref_num: u64,
    pub quantity: u32,
    // TODO pub stock: StockSymbol,
    // pub price: Price32,
    pub match_number: u64,
}

impl MatchTrade {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        // Side flag will always be "B", so we can ignore it.
        let (input, quantity) = be_u32(input)?;
        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;
        let (input, match_number) = be_u64(input)?;

        Ok((input, Self { 
            order_ref_num,
            quantity,
            // stock
            // price
            match_number,
        }))
    }

}


/// NASDAQ sends out this message for all active issues in the system following
/// the Opening, Closing and Extended Market Close cross events when the 
/// cross process for that security has been completed. 
/// 
/// To avoid double counting of cross volume, firms should not include 
/// non-printable transactions into time-and-sales displays or market 
/// statistic calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrossTrade {
    pub order_ref_num: u64,
    /// For most issues, `quantity` is volume associated with the cross event. 
    /// If the order interest is insufficient to conduct a cross, 
    /// this may appear as zero.
    pub quantity: u32,
    // TODO pub stock: StockSymbol,
    // pub price: Price32,
    pub match_number: u64,
    pub cross_type: CrossType,
}

impl CrossTrade {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, order_ref_num) = be_u64(input)?;
        let (input, quantity) = be_u32(input)?;
        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;
        let (input, match_number) = be_u64(input)?;
        let (input, cross_type) = CrossType::parse(input)?;

        Ok((input, Self { 
            order_ref_num,
            quantity,
            // stock
            // price
            match_number,
            cross_type 
        }))
    }

}


/// Sent whenever an execution on Nasdaq is broken. 
/// An execution may be broken if it is found to be “clearly erroneous” 
/// pursuant to Nasdaq’s Clearly Erroneous Policy. 
/// A trade break is final; once a trade is broken, it cannot be reinstated.
/// 
/// Firms that use the ITCH feed to create time-and-sales displays or calculate 
/// market statistics should be prepared to process this message. 
/// If a firm is only using the ITCH feed to build a book, however, 
/// it may ignore these messages as they have no impact on the current book.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrokenTrade {
    pub match_number: u64,
}

impl BrokenTrade {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, match_number) = be_u64(input)?;
        Ok((input, Self { match_number }))
    }

}


crate::types::define_enum!{

    CrossType:
        "NASDAQ cross session for which the message is being generated.";

    ['O'] Opening
        "NASDAQ opening cross",
    ['C'] Closing
        "NASDAQ closing cross",
    ['H'] Halt
        "Cross for IPO and halted / paused securities.",
}


