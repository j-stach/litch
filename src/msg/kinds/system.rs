
use nom::number::streaming::be_u64;
use nsdq_util::{ 
    define_enum, 
    StockSymbol, 
    Price,
    NaiveTime,
    parse_itch_time_bold,
    parse_bool,
};

// NOTE: SystemEvent is a message type that conveys a single tag.
define_enum!{

    SystemEvent:
        "System events that can occur on the TotalView-ITCH data feed.";

    ['O'] BeginMessages 
        "Start of Messages. \n \
        This is the first message sent in any trading day. \
        (With the exception of Timestamp messages.)",

    ['S'] BeginSystemHours 
        "Start of System hours. \n \
        NASDAQ is open and ready to start accepting orders.",

    ['Q'] BeginMarketHours
        "Start of Market hours. \n \
        Market Hours orders are available for execution.",

    ['M'] EndMarketHours 
        "End of Market hours. 
        Market Hours orders are no longer available for execution.",

    ['E'] EndSystemHours
        "End of System hours. \n \
        NASDAQ has closed and will not accept any new orders.
        NOTE: It is still possible to receive BrokenTrade and OrderDelete 
        messages after the End of Day.",

    ['C'] EndMessages 
        "End of Messages. \n \
        This is always the last message sent in any trading day.",
}


/// Nasdaq begins disseminating Net Order Imbalance Indicators (NOII) 
/// at 9:25 a.m. for the Opening Cross and 3:50 p.m. for the Closing Cross.
///
/// Between 9:25 and 9:28 a.m. and 3:50 and 3:55 p.m., 
/// Nasdaq disseminates the NOII information every 10 seconds.
///
/// Between 9:28 and 9:30 a.m. and 3:55 and 4:00 p.m., Nasdaq disseminates the 
/// NOII information every second.
///
/// For Nasdaq Halt, IPO and Pauses, NOII messages will be disseminated at 
/// 1 second intervals starting 1 second after quoting period starts and 
/// trading action is released.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetOrderImbalance {

    /// Number of shares that can be matched at the Current Reference Price.
    pub paired_shares: u64,
    /// The number of shares not paired at the Current Reference Price.
    pub imbalance_shares: u64,
    /// The market side of the order imbalance.
    pub imbalance_direction: ImbalanceDirection,
    /// Symbol for the security being listed.
    pub stock: StockSymbol,
    /// Hypothetical auction-clearing price (for cross orders only).
    pub far_price: Price<u32, 4>,
    /// Hypothetical auction-clearing price 
    /// (for cross orders as well as continuous orders).
    pub near_price: Price<u32, 4>,
    /// Price at which the NOII shares are being calculated.
    pub ref_price: Price<u32, 4>,
    /// The type of cross for which the NOII message is being generated.
    pub cross_type: ImbalanceCrossType,
    /// Absolute value of the percentage of deviation of the 
    /// Near Indicative Clearing Price to the nearest Current Reference Price.
    pub price_variation: PriceVariation,
}

impl NetOrderImbalance {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, paired_shares) = be_u64(input)?;
        let (input, imbalance_shares) = be_u64(input)?;
        let (input, imbalance_direction) = ImbalanceDirection::parse(input)?;
        let (input, stock) = StockSymbol::parse(input)?;
        let (input, far_price) = Price::<u32, 4>::parse(input)?;
        let (input, near_price) = Price::<u32, 4>::parse(input)?;
        let (input, ref_price) = Price::<u32, 4>::parse(input)?;
        let (input, cross_type) = ImbalanceCrossType::parse(input)?;
        let (input, price_variation) = PriceVariation::parse(input)?;

        Ok((input, Self {
            paired_shares,
            imbalance_shares,
            imbalance_direction, 
            stock,
            far_price,
            near_price,
            ref_price,
            cross_type,
            price_variation,
        }))
    }

}

define_enum!{

    ImbalanceDirection:
        "The market side of the order imbalance.";

    ['B'] Buy,
    ['S'] Sell,
    ['N'] NoImbalance,
    ['O'] Uncalculated,
    ['P'] Paused,
}

// TODO: Consolidate cross types?
define_enum!{

    ImbalanceCrossType: "";

    ['O'] Opening,
    ['C'] Closing,
    ['H'] Halt,
    ['A'] ExtendedClose,
}

define_enum!{

    PriceVariation:
        "Absolute value of the percentage of deviation of the \
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

/// Identifies a retail interest indication of the Bid, Ask, 
/// or both the Bid and Ask for NASDAQ-listed securities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetailPriceImprovement {

    /// Symbol for the RPI security.
    pub stock: StockSymbol,
    /// Availability of Retail Price Improvement orders.
    pub interest_flag: InterestFlag
}

impl RetailPriceImprovement {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, stock) = StockSymbol::parse(input)?;
        let (input, interest_flag) = InterestFlag::parse(input)?;

        Ok((input, Self { stock, interest_flag }))
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


/// Sent only for Direct Listing with Capital Raise (DLCR) securities. 
/// Nasdaq begins disseminating messages once per second as soon as the 
/// DLCR volatility test has successfully passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectListingWithCapitalRaise {

    /// Symbol for the security listed.
    pub stock: StockSymbol,
    /// Indicates if the security is eligible to be released for trading.
    pub eligibility: bool,
    /// 20% below Registration Statement Lower Price.
    pub min_price: Price<u32, 4>,
    /// 80% above Registration Statement Highest Price.
    pub max_price: Price<u32, 4>,
    /// The current reference price when the DLCR volatility test has 
    /// successfully passed.
    pub near_exec_price: Price<u32, 4>,
    /// The time at which the near execution price was set.
    pub near_exec_time: NaiveTime,
    /// Indicates the price of the Lower Auction Collar Threshold
    /// (10% below the Near Execution Price).
    pub lower_collar: Price<u32, 4>,
    /// Indicates the price of the Upper Auction Collar Threshold
    /// (10% above the Near Execution Price).
    pub upper_collar: Price<u32, 4>,
}

impl DirectListingWithCapitalRaise {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, stock) = StockSymbol::parse(input)?;
        let (input, eligibility) = parse_bool(input)?;
        let (input, min_price) = Price::<u32, 4>::parse(input)?;
        let (input, max_price) = Price::<u32, 4>::parse(input)?;
        let (input, near_exec_price) = Price::<u32, 4>::parse(input)?;
        let (input, near_exec_time) = parse_itch_time_bold(input)?;
        let (input, lower_collar) = Price::<u32, 4>::parse(input)?;
        let (input, upper_collar) = Price::<u32, 4>::parse(input)?;

        Ok((input, Self { 
            stock,
            eligibility,
            min_price,
            max_price,
            near_exec_price,
            near_exec_time,
            lower_collar,
            upper_collar,
        }))
    }

}

