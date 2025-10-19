
mod metadata;
mod kinds;

pub use metadata::ItchMetadata;
pub use kinds::*;


//
macro_rules! parse_kind {
    ($input:expr, $meta:expr, $kind:ident) => {{
        let (input, body) = $kind::parse($input)?;
        (input, Self::$kind { metadata: $meta, body })
    }}
}

//
macro_rules! msg_kinds {
    ($([$tag:expr] $kind:ident $doc:expr),* $(,)?) => {

        /// Represents all messages that can originate from an ITCH broadcast.
        /// "Message Type" tag is implicit through this enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ItchMessage {$(
            #[doc = $doc]
            $kind { metadata: ItchMetadata, body: $kind },
        )*}

        impl ItchMessage {
            // TODO ItchError::Parse and UnexpectedTag, etc.
            pub fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

                let (input, tag) = nom::number::streaming::be_u8(input)?;
                let (input, metadata) = ItchMetadata::parse(input)?;
                let (input, message) = match tag {
                    $(
                        $tag => parse_kind!(input, metadata, $kind),
                    )*
                    _ => unimplemented!{}
                };

                Ok((input, message))
            }

            // TODO get metadata values with convenience
        }

    }
}

msg_kinds!{

    // 1.1
    [b'S'] SystemEvent 
        "Market or data feed handler event.",

    // 1.2
    [b'R'] StockDirectory
        "",
    [b'H'] TradingAction 
        "",            
    [b'Y'] RegShoRestriction 
        "Regulation SHO Short-Sale-Price-Test-Restricted Indicator.",       
    [b'L'] MarketParticipantPosition
        "",
    [b'V'] MwcbDeclineLevel
        "",
    [b'W'] MwcbStatus                  
        "",
    [b'K'] QuotingPeriodUpdate         
        "",
    [b'J'] LuldAuctionCollar            
        "",
    [b'h'] OperationalHalt             
        "",

    // 1.3
    [b'A'] AddOrder                    
        "",
    [b'F'] AddOrderWithAttr            
        "",

    // 1.4
    [b'E'] OrderExecuted            
        "",
    [b'C'] OrderExecutedWithPrice      
        "",
    [b'X'] OrderCancel
        "",
    [b'D'] OrderDelete
        "",
    [b'U'] OrderReplace
        "",
    
    // 1.5
    // [b'P'] MatchTrade
    // [b'Q'] CrossTrade
    // [b'B'] BrokenTrade
    
    // 1.6
    // [b'I'] NetOrderImbalance
    
    // 1.7
    // [b'N'] RetailPriceImprovement
    
    // 1.8
    // [b'O'] DirectListingWithCapitalRaise

}


