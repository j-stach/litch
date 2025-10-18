
use crate::types::EventCode;

/// Signals a market or data feed handler event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemEvent {
    // TODO: TBD: SystemEvent & EventCode can be collapsed into one type?
    pub event_code: EventCode 
}

impl SystemEvent {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, event_code) = EventCode::parse(input)?;
        Ok((input, Self { event_code }))
    }

}
