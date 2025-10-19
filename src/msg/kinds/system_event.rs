
/// Signals a market or data feed handler event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemEvent {
    // TODO: Can be collapsed into one type?
    pub event_code: EventCode 
}

impl SystemEvent {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, event_code) = EventCode::parse(input)?;
        Ok((input, Self { event_code }))
    }

}

crate::types::define_enum!{

    EventCode:
        "System events that can occur on the TotalView-ITCH data feed.";

    ['O'] BeginMessages 
        "Start of Messages. 
        This is the first message sent in any trading day.
        (With the exception of Timestamp messages.)",

    ['S'] BeginSystemHours 
        "Start of System hours. 
        NASDAQ is open and ready to start accepting orders.",

    ['Q'] BeginMarketHours
        "Start of Market hours. 
        Market Hours orders are available for execution.",

    ['M'] EndMarketHours 
        "End of Market hours. 
        Market Hours orders are no longer available for execution.",

    ['E'] EndSystemHours
        "End of System hours. 
        NASDAQ has closed and will not accept any new orders.
        NOTE: It is still possible to receive BrokenTrade and OrderDelete 
        messages after the End of Day.",

    ['C'] EndMessages 
        "End of Messages. 
        This is always the last message sent in any trading day.",
}

