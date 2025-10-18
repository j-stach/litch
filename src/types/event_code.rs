
/// System Events that occur on the TotalView-ITCH data feed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCode {
    /// Start of Messages. 
    /// This is the first message sent in any trading day.
    /// (With the exception of Timestamp messages.)
    BeginMessages,

    /// Start of System hours. 
    /// NASDAQ is open and ready to start accepting orders.
    BeginSystemHours,

    /// Start of Market hours. 
    /// Market Hours orders are available for execution.
    BeginMarketHours,

    /// End of Market hours. 
    /// Market Hours orders are no longer available for execution.
    EndMarketHours,

    /// End of System hours. 
    /// NASDAQ has closed and will not accept any new orders.
    /// NOTE: It is still possible to receive BrokenTrade and OrderDelete 
    /// messages after the End of Day.
    EndSystemHours,

    /// End of Messages. 
    /// This is always the last message sent in any trading day.
    EndMessages,

}

use nom::{ Parser, branch::alt, combinator::map, character::char };

impl EventCode {
    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, event_code) = alt((
            map(char('O'), |_| Self::BeginMessages),
            map(char('S'), |_| Self::BeginSystemHours),
            map(char('Q'), |_| Self::BeginMarketHours),
            map(char('M'), |_| Self::EndMarketHours),
            map(char('E'), |_| Self::EndSystemHours),
            map(char('C'), |_| Self::EndMessages),
        )).parse(input)?;

        Ok((input, event_code))
    }
}
