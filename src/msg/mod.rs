
mod metadata;
mod kinds;

pub use metadata::ItchMetadata;
pub use kinds::*;


/// Represents all messages that can originate from an ITCH broadcast.
/// "Message Type" tag is implicit through this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItchMessage {

    /// Signals a market or data feed handler event.
    SystemEvent { metadata: ItchMetadata, body: SystemEvent },

}

use nom::number::streaming::be_u8;

impl ItchMessage {

    // TODO ItchError::Parse and UnexpectedTag, etc.
    pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, tag) = be_u8(input)?;
        let (input, metadata) = ItchMetadata::parse(input)?;
        let (input, message) = match tag {

            b'S' => (input, Self::SystemEvent { 
                metadata, 
                body: SystemEvent::parse(input)?.1, 
            }),

            _ => unimplemented!{}
        };

        Ok((input, message))
    }

    // TODO get metadata values with convenience
}

