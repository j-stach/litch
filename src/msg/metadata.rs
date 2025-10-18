
use chrono::NaiveTime;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItchMetadata {

    /// Integer uniquely assigned to the security symbol (updated daily).
    pub stock_locate: u16,

    /// NASDAQ internal tracking number. (OrderRefNum in OUCH)
    pub tracking_number: u16,

    /// Time this message was generated.
    pub timestamp: NaiveTime,
}

use nom::number::streaming::{ be_u16, be_u64 };
use crate::helper::nanosec_from_midnight;

impl ItchMetadata {
    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        //
        let (input, stock_locate) = be_u16(input)?;
        let (input, tracking_number) = be_u16(input)?;
        let (input, timestamp) = be_u64(input)?;

        Ok((input, Self { 
            stock_locate, 
            tracking_number, 
            timestamp: nanosec_from_midnight(timestamp)
        }))
    }
}


