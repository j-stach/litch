
use nsdq_util::define_enum;

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

define_enum!{

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


/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetOrderImbalance {
    // paired_shares u64
    // imbalance_shares u6
    pub imbalance_direction: ImbalanceDirection,
    // stock
    // far_price: Price32,
    // near_price: Price32,
    // ref_price: Price32,
    pub cross_type: ImbalanceCrossType,
    pub price_variation: PriceVariation,
}

impl NetOrderImbalance {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        let (input, imbalance_direction) = ImbalanceDirection::parse(input)?;
        let (input, cross_type) = ImbalanceCrossType::parse(input)?;
        let (input, price_variation) = PriceVariation::parse(input)?;

        Ok((input, Self { 
            imbalance_direction, 
            /*stock,*/ 
            cross_type,
            price_variation,
        }))
    }

}

define_enum!{

    ImbalanceDirection:
        "";

    ['B'] Buy
        "",
    ['S'] Sell
        "",
    ['N'] NoImbalance
        "",
    ['O'] Uncalculated
        "",
    ['P'] Paused
        "",
}

define_enum!{

    ImbalanceCrossType:
        "";

    ['O'] Opening
        "",
    ['C'] Closing
        "",
    ['H'] Halt
        "",
    ['A'] ExtendedClose
        "",
}

define_enum!{

    PriceVariation:
        "Absolute value of the percentage of deviation of the 
        Near Indicative Clearing Price to the nearest Current Reference Price.";

    ['L'] Zero
        "Less than 1%",
    ['1'] One
        "1 to 1.99%",
    ['2'] Two
        "2 to 2.99%",
    ['3'] Three
        "3 to 3.99%",
    ['4'] Four
        "4 to 4.99%",
    ['5'] Five
        "5 to 5.99%",
    ['6'] Six
        "6 to 6.99%",
    ['7'] Seven
        "7 to 7.99%",
    ['8'] Eight
        "8 to 8.99%",
    ['9'] Nine
        "9 to 9.99%",
    ['A'] Ten
        "10 to 19.99%",
    ['B'] Twenty
        "20 to 29.99%",
    ['C'] ThirtyUp
        "30% or greater",
    [' '] Uncalculated
        "Cannot be calculated",
}

/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetailPriceImprovement {
    // stock
    pub interest_flag: InterestFlag
}

impl RetailPriceImprovement {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        let (input, interest_flag) = InterestFlag::parse(input)?;

        Ok((input, Self { interest_flag }))
    }

}

define_enum!{

    InterestFlag:
        "Availability of Retail Price Improvement orders.";

    ['B'] BuyAvailable
        "RPI orders available on the buy side.",
    ['S'] SellAvailable
        "RPI orders available on the sell side.",
    ['A'] AnyAvailable
        "RPI orders available on both sides (buy and sell).",
    ['N'] NoneAvailable
        "No RPI orders available.",
}


/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectListingWithCapitalRaise {
    // stock
    // pub open_eligibility_status: bool,
    // min allowable price: Price32
    // max allowable price: Price32
    // near execution price: Price32
    // near execution time: u64
    // lower price range collar: Price32
    // upper price range collar: Price32
}

impl DirectListingWithCapitalRaise {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO

        Ok((input, Self {  }))
    }

}

