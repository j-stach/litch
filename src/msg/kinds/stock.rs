
use nom::number::streaming::be_u32;
use nsdq_util::{ 
    define_enum,
    parse_bool,
    parse_ternary,
    StockSymbol,
    NaiveTime,
    parse_nanosecs_bold,
    Price,
    Mpid,
};

/// At the start of each trading day, Nasdaq disseminates stock directory 
/// messages for all active symbols in their execution system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StockDirectory {

    /// Security symbol for the issue in the NASDAQ execution system.
    pub stock: StockSymbol,

    /// Listing market or listing market tier for the issue.
    pub market_category: MarketCategory,

    /// Firm compliance with continued listing requirements.
    pub financial_status: FinancialStatus,

    /// Number of shares that represent a round lot for the issue.
    pub round_lot_size: u32,

    /// Indicates if Nasdaq system limits order entry for issue.
    pub round_lots_only: bool,

    /// Identifies the security class for the issue.
    pub class: IssueClassification,

    // TODO pub subtype: IssueSubType, // 2byte alpha

    /// Denotes if an issue or quoting participant record is set-up in a 
    /// live/production, test, or demo state.
    /// Please note that firms should only show live issues and quoting
    /// participants on public quotation displays.
    pub authenticity: Authenticity,

    /// If `Some(true)`, the security is subject to mandatory close-out of 
    /// short sales under SEC Rule 203(b)(3).
    /// If `None`, the information is not available.
    pub short_sale_threshold: Option<bool>,

    /// This field is intended to help Nasdaq market participant firms comply 
    /// with FINRA Rule 5131(b).
    /// If `Some(true)`, the security is set up for IPO release. 
    /// If `None`, the information is not available.
    pub ipo_flag: Option<bool>,

    /// Indicates which Limit Up / Limit Down price band calculation parameter 
    /// is to be used for the instrument. 
    pub luld_tier: LuldTier,

    /// If `Some(true)`, the security is an exchange traded product (ETP).
    /// If `None`, the information is not available.
    pub etp_flag: Option<bool>,

    /// Tracks the integral relationship of the ETP to the underlying index.
    ///
    /// (Example: If the underlying Index increases by a value of 1 and
    /// the ETP’s Leverage factor is 3, indicates the ETF will
    /// increase/decrease (see Inverse) by 3.)
    ///
    /// Leverage Factor is rounded to the nearest integer below, 
    /// e.g., leverage factor 1 would represent leverage factors of 1 to 1.99.
    ///
    /// This field is used for LULD Tier I price band calculation purposes.
    pub etp_leverage_factor: u32,

    /// Direction of the relationship between the ETP and Underlying index.
    /// If `true`, ETP is an Inverse ETP.
    pub inverse: bool,
}

impl StockDirectory {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, stock) = StockSymbol::parse(input)?;
        let (input, market_category) = MarketCategory::parse(input)?;
        let (input, financial_status) = FinancialStatus::parse(input)?;
        let (input, round_lot_size) = be_u32(input)?;
        let (input, round_lots_only) = parse_bool(input)?;
        let (input, class) = IssueClassification::parse(input)?;
        let (input, authenticity) = Authenticity::parse(input)?;
        let (input, short_sale_threshold) = parse_ternary(input)?;
        let (input, ipo_flag) = parse_ternary(input)?;
        let (input, luld_tier) = LuldTier::parse(input)?;
        let (input, etp_flag) = parse_ternary(input)?;
        let (input, etp_leverage_factor) = be_u32(input)?;
        let (input, inverse) = parse_bool(input)?;

        // TODO Other fields

        Ok((input, Self { 
            stock,
            market_category,
            financial_status,
            round_lot_size,
            round_lots_only,
            class,
            authenticity,
            short_sale_threshold,
            ipo_flag,
            luld_tier,
            etp_flag,
            etp_leverage_factor,
            inverse,
        }))
    }

}

define_enum!{

    MarketCategory:
        "Listing market or listing market tier for the issue.";

    ['Q'] NasdaqGlobalSelectMarket
        "Nasdaq Global Select Market",
    ['G'] NasdaqGlobalMarket
        "Nasdaq Global Market",
    ['S'] NasdaqCapitalMarket
        "Nasdaq Capital Market",
    ['N'] Nyse
        "New York Stock Exchange (NYSE)",
    ['A'] NyseAmerican
        "NYSE American",
    ['P'] NyseArca
        "NYSE Arca",
    ['Z'] BatsZExchange
        "BATS Z Exchange",
    ['V'] InvestorsExchange
        "Investors’ Exchange, LLC",
    [' '] NotAvailable
        "Not available.",
}

define_enum!{

    FinancialStatus:
        "Status of firm compliance with Nasdaq continued listing requirements.";

    ['D'] Deficient
        "Deficient",
    ['E'] Delinquent
        "Delinquent",
    ['Q'] Bankrupt
        "Bankrupt",
    ['S'] Suspended
        "Suspended",
    ['G'] DeficientBankrupt
        "Deficient and bankrupt.",
    ['H'] DeficientDelinquent
        "Deficient and delinquent.",
    ['J'] DelinquentBankrupt
        "Delinquent and bankrupt.",
    ['K'] DeficientDelinquentBankrupt
        "Deficient, delinquent, and bankrupt.",
    ['C'] CreateRedeemSuspended
        "Creations and/or Redemptions Suspended for Exchange Traded Product.",
    ['N'] Compliant
        "Normal/default. Not delinquent, deficient, or bankrupt.",
    [' '] NotAvailable
        "Not available. Firms should refer to SIAC feeds for code if needed.",
}

define_enum! {

    IssueClassification:
        "Identifies the security class for the issue as assigned by Nasdaq.";

    ['A'] AmericanDepositaryShare
        "",
    ['B'] Bond
        "",
    ['C'] CommonStock
        "",
    ['F'] DepositoryReceipt
        "",
    ['I'] _144A
        "",
    ['L'] LimitedPartnership
        "",
    ['N'] Notes
        "",
    ['O'] OrdinaryShare
        "",
    ['P'] PreferredStock
        "",
    ['Q'] OtherSecurities
        "",
    ['R'] Right
        "",
    ['S'] SharesOfBeneficialInterest
        "",
    ['T'] ConvertibleDebenture
        "",
    ['U'] Unit
        "",
    ['V'] UnitsBenifInt
        "",
    ['W'] Warrant
        "",
}

define_enum!{

    Authenticity:
        "Denotes if an issue or quoting participant record is set-up in
        Nasdaq systems in a live/production, test, or demo state.
        NOTE: Firms should only show live issues and quoting
        participants on public quotation displays.";

    ['P'] Production
        "Live/Production",
    ['T'] Test
        "Test only",
}

define_enum!{

    LuldTier:
        "Indicates which Limit Up / Limit Down price band calculation
        parameter is to be used for the instrument. 
        Refer to LULD Rule for details.";

    ['1'] Tier1
        "Tier 1 NMS Stocks and select ETPs.",
    ['2'] Tier2
        "Tier 2 NMS Stocks.",
    [' '] NotAvailable
        "Not available.",
}


/// In the pre-market spin, Nasdaq will send out a Trading Action message 
/// with the `TradingState::Trading` for all listed securities that are eligible
/// for trading at the start of the system hours. 
///
/// If a security is absent from the pre-opening Trading Action spin, 
/// assume that the security is being treated as halted in the Nasdaq
/// platform at the start of the system hours.
///
/// During the day, Nasdaq will use the Trading Action message to relay changes 
/// in trading status for an individual security.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TradingAction {

    /// Issue that has been affected by the trading action.
    pub stock: StockSymbol,
    /// Current trading state of the stock.
    pub state: TradingState,
    /// Reserved. 
    // TBD: Unclear in the spec exactly what this does or how it is used.
    pub reserved: char,
    //TODO reason: TradingActionReason (string4)
}

impl TradingAction {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, stock) = StockSymbol::parse(input)?;
        let (input, state) = TradingState::parse(input)?;
        let (input, rsvd) = nom::bytes::complete::take(1usize)(input)?;

        Ok((input, Self { 
            stock, 
            state, 
            reserved: rsvd[0] as char,
        }))
    }

}

define_enum!{

    TradingState:
        "Current trading state of the stock.";

    ['H'] Halted 
        "Halted across all U.S. equity markets / SROs.",
    ['P'] Paused
        "Paused across all U.S. equity markets / SROs 
        (Nasdaq---listed securities only).",
    ['Q'] QuoteOnly
        "Quotation only period for cross-SRO halt or pause.",
    ['T'] Trading
        "Trading on NASDAQ.",
}


/// Reg SHO Short Sale Price Test Restricted Indicator.
/// Pre-opening spin indicates the Rule 201 status for all active issues. 
/// Nasdaq also sends this message in the event of an intraday status change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegShoRestriction {

    /// Issue to be described
    pub stock: StockSymbol,
    /// Restriction status
    pub action: RegShoAction,
}

impl RegShoRestriction {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, stock) = StockSymbol::parse(input)?;
        let (input, action) = RegShoAction::parse(input)?;

        Ok((input, Self { stock, action }))
    }

}

define_enum!{
    RegShoAction: 
        "Regulation SHO Short Sale Price Test Restriction status.";

    ['0'] NoPriceTest 
        "No price test in place.",
    ['1'] PriceDrop 
        "Restriction in effect due to an intra-day price drop in security.",
    ['2'] RemainInEffect 
        "Restriction remains in effect.",
}


/// Provides the position of each market participant registered in an issue.
/// Typically sent in the spin at the start of day.
/// During the day, this message will only be sent if Nasdaq Operations 
/// manually changes the status of a market participant firm in an issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketParticipantPosition {

    /// Identifies the market participant firm.
    pub mpid: Mpid,
    /// Identifies the issue the firm has a position in.
    pub stock: StockSymbol,
    /// If `true`, firm qualifies as a Primary Market Maker.
    pub is_primary: bool,
    /// Quoting participant’s registration status in relation to 
    /// SEC Rules 101 and 104 of Regulation M.
    pub mode: MarketMakerMode,
    /// Market participant’s current registration status in the issue.
    pub state: MarketParticipantState,

}

impl MarketParticipantPosition {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, mpid) = Mpid::parse(input)?;
        let (input, stock) = StockSymbol::parse(input)?;
        let (input, is_primary) = parse_bool(input)?;
        let (input, mode) = MarketMakerMode::parse(input)?;
        let (input, state) = MarketParticipantState::parse(input)?;

        Ok((input, Self { mpid, stock, is_primary, mode, state }))
    }

}

define_enum!{

    MarketMakerMode:
        "Quoting participant’s registration status in relation to SEC Rules 101 
        and 104 of Regulation M.";

    ['N'] Normal
        "Normal",
    ['P'] Passive
        "Passive",
    ['S'] Syndicate
        "Syndicate",
    ['R'] PreSyndicate
        "Pre-syndicate",
    ['L'] Penalty
        "Penalty",
}

define_enum!{

    MarketParticipantState:
        "Market participant’s current registration status in the issue.";

    ['A'] Active
        "Active",
    ['E'] Excused
        "Excused/Withdrawn",
    ['W'] Withdrawn
        "Withdrawn",
    ['S'] Suspended
        "Suspended",
    ['D'] Deleted
        "Deleted",
}


/// Market-wide Circuit Breaker (MWCB) breach points for the current trading day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MwcbDeclineLevel {

    pub level_1: Price<u64, 8>,
    pub level_2: Price<u64, 8>,
    pub level_3: Price<u64, 8>,
}

impl MwcbDeclineLevel {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, level_1) = Price::<u64, 8>::parse(input)?;
        let (input, level_2) = Price::<u64, 8>::parse(input)?;
        let (input, level_3) = Price::<u64, 8>::parse(input)?;

        Ok((input, Self { level_1, level_2, level_3 }))
    }

}


/// Sent when a Market-wide Circuit Breaker (MWCB) level has been breached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MwcbStatus {

    /// Denotes the MWCB Level that was breached.
    pub level: BreachedLevel
}

impl MwcbStatus {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, level) = BreachedLevel::parse(input)?;
        Ok((input, Self { level }))
    }
}

define_enum!{

    BreachedLevel:
        "Denotes the MWCB Level that was breached.";

    ['1'] _1
        "Level 1",
    ['2'] _2
        "Level 2",
    ['3'] _3
        "Level 3",
}


/// Indicates the anticipated IPO quotation release time of a security.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuotingPeriodUpdate {

    /// Timestamp of future release.
    pub release_time: NaiveTime,
    /// Status of the pending IPO release.
    pub qualifier: IpoQuotationReleaseQualifier,
    /// Price of the IPO.
    pub ipo_price: Price<u32, 4>,
}

impl QuotingPeriodUpdate {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, release_time) = parse_nanosecs_bold(input)?;
        let (input, qualifier) = IpoQuotationReleaseQualifier::parse(input)?;
        let (input, ipo_price) = Price::<u32, 4>::parse(input)?;

        Ok((input, Self { 
            release_time,
            qualifier, 
            ipo_price,
        }))
    }

}

define_enum!{

    IpoQuotationReleaseQualifier:
        "Status of the pending IPO release";

    ['A'] Anticipated
        "Nasdaq Market Operations has entered the IPO instrument for release",
    ['C'] Canceled
        "Nasdaq Market Operations has canceled or postponed the IPO release.",
}


/// Auction collar thresholds within which a paused security can reopen 
/// following a Limit-Up/Limit-down (LULD) Trading Pause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LuldAuctionCollar {

    /// Security which was paused.
    pub stock: StockSymbol,
    /// Reference price used to set the Auction Collars.
    pub reference_price: Price<u32, 4>,
    /// Upper Auction Collar Threshold
    pub upper_price: Price<u32, 4>,
    /// Lower Auction Collar Threshold
    pub lower_price: Price<u32, 4>,
    /// Indicates the number of the extensions to the Reopening Auction.
    pub extension: u32,
}

impl LuldAuctionCollar {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, stock) = StockSymbol::parse(input)?;
        let (input, reference_price) = Price::<u32, 4>::parse(input)?;
        let (input, upper_price) = Price::<u32, 4>::parse(input)?;
        let (input, lower_price) = Price::<u32, 4>::parse(input)?;
        let (input, extension) = be_u32(input)?;

        Ok((input, Self {
            stock,
            reference_price,
            upper_price,
            lower_price,
            extension
        }))
    }
}


/// Interruption of service on `stock` impacting only the designated `market`. 
///
/// These Halts differ from the `TradingAction` message types since an 
/// Operational Halt is specific to the exchange for which it is declared, 
/// and does not interrupt the identified instrument on any other marketplace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalHalt {

    /// Instrument for which the halt was imposed.
    pub stock: StockSymbol,
    /// Exchange on which the stock is halted.
    pub market: MarketCode,
    /// Status of the halted market.
    pub action: HaltAction,
}

impl OperationalHalt {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, stock) = StockSymbol::parse(input)?;
        let (input, market) = MarketCode::parse(input)?;
        let (input, action) = HaltAction::parse(input)?;

        Ok((input, Self { stock, market, action }))
    }
}

define_enum!{

    MarketCode:
        "Market being halted:";

    ['Q'] Nasdaq
        "NASDAQ",
    ['B'] Bx
        "BX",
    ['X'] Psx
        "PSX"
}

define_enum!{

    HaltAction:
        "Market halt status:";

    ['H'] Halted
        "Operationally Halted on the identified Market.",
    ['T'] Trading
        "Operational Halt has been lifted and Trading resumed.",
}

