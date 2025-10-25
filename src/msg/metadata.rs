
use nsdq_util::NaiveTime;

/// Data common to all ITCH message types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItchMetadata {

    /// Integer uniquely assigned to the security symbol (updated daily).
    pub stock_locate: u16,

    /// NASDAQ internal tracking number.
    pub tracking_number: u16,

    /// Time this message was generated.
    pub timestamp: NaiveTime,
}

use nom::number::streaming::be_u16;
use nsdq_util::parse_itch_time_bold;

impl ItchMetadata {
    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, stock_locate) = be_u16(input)?;
        let (input, tracking_number) = be_u16(input)?;
        let (input, timestamp) = parse_itch_time_bold(input)?;

        Ok((input, Self { 
            stock_locate, 
            tracking_number, 
            timestamp,
        }))
    }
}


