
use nom::number::streaming::be_u32;

///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StockDirectory {
    //TODO stock: StockSymbol,
    pub market_category: MarketCategory,
    pub financial_status: FinancialStatus,
    pub round_lot_size: u32,
    // TODO pub round_lots_only: bool,
    // TODO pub classification: IssueClassification,
    // TODO pub subtype: IssueSubType,
    pub authenticity: Authenticity,
    pub short_sale_threshold: ShortSaleThreshold,
    pub ipo_flag: IpoFlag,
    pub luld_tier: LuldTier,
    pub etp_flag: EtpFlag,
    // TODO pub etp_leverage_factor: EtpLeverageFactor,
    // TODO pub inverse: bool,
    
}

impl StockDirectory {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, market_category) = MarketCategory::parse(input)?;
        let (input, financial_status) = FinancialStatus::parse(input)?;
        let (input, round_lot_size) = be_u32(input)?;
        let (input, authenticity) = Authenticity::parse(input)?;
        let (input, short_sale_threshold) = ShortSaleThreshold::parse(input)?;
        let (input, ipo_flag) = IpoFlag::parse(input)?;
        let (input, luld_tier) = LuldTier::parse(input)?;
        let (input, etp_flag) = EtpFlag::parse(input)?;

        // TODO Other fields

        Ok((input, Self { 
            market_category,
            financial_status,
            round_lot_size,
            authenticity,
            short_sale_threshold,
            ipo_flag,
            luld_tier,
            etp_flag
        }))
    }

}

crate::types::define_enum!{

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

crate::types::define_enum!{

    FinancialStatus:
        "Status of firm compliance with Nasdaq continued listing requirements.";

    ['D'] Deficient
        "",
    ['E'] Delinquent
        "",
    ['Q'] Bankrupt
        "",
    ['S'] Suspended
        "",
    ['G'] DeficientBankrupt
        "",
    ['H'] DeficientDelinquent
        "",
    ['J'] DelinquentBankrupt
        "",
    ['K'] DeficientDelinquentBankrupt
        "",
    ['C'] CreateRedeemSuspended
        "Creations and/or Redemptions Suspended for Exchange Traded Product.",
    ['N'] Compliant
        "Normal/default. Not delinquent, deficient, or bankrupt.",
    [' '] NotAvailable
        "Not available. Firms should refer to SIAC feeds for code if needed.",
}

crate::types::define_enum!{

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

crate::types::define_enum!{

    ShortSaleThreshold:
        "Indicates if a security is subject to mandatory close-out of
        short sales under SEC Rule 203(b)(3).";

    ['Y'] Yes
        "Issue is restricted under SEC Rule 203(b)(3).",
    ['N'] No
        "Issue is not restricted.",
    [' '] NotAvailable
        "Not available.",
}

crate::types::define_enum!{

    IpoFlag:
        "Indicates if the security is set up for IPO release. 
        This field is intended to help Nasdaq market participant firms comply 
        with FINRA Rule 5131(b).";

    ['Y'] Yes
        "NASDAQ-listed instrument is set up as a new IPO security.",
    ['N'] No
        "NASDAQ-listed instrument is not set up as a new IPO security.",
    [' '] NotAvailable
        "Not available.",
}

crate::types::define_enum!{

    LuldTier:
        "Indicates which Limit Up / Limit Down price band calculation
        parameter is to be used for the instrument. 
        Refer to LULD Rule for details.";

    ['1'] Tier1
        "Tier 1 NMS Stocks and select ETPs.",
    ['N'] Tier2
        "Tier 2 NMS Stocks.",
    [' '] NotAvailable
        "Not available.",
}

crate::types::define_enum!{

    EtpFlag:
        "Indicates whether the security is an exchange traded product (ETP).";

    ['Y'] Yes
        "Instrument is an ETP.",
    ['N'] No
        "Instrument is not an ETP.",
    [' '] NotAvailable
        "Not available",
}


///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TradingAction {
    //TODO stock: StockSymbol,
    /// Current trading state of the stock.
    pub state: TradingState,
    //TODO pub reserved: char,
    //TODO reason: TradingActionReason (string4)
}

impl TradingAction {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, state) = TradingState::parse(input)?;
        // TODO Other fields

        Ok((input, Self { state }))
    }

}

crate::types::define_enum!{

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
    //TODO: Convert slouch to use nom for parsing responses
    //pub stock: StockSymbol,
    pub action: RegShoAction,
}

impl RegShoRestriction {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;
        let (input, action) = RegShoAction::parse(input)?;

        Ok((input, Self { /*stock,*/ action }))
    }

}

crate::types::define_enum!{
    RegShoAction: 
        "Regulation SHO Short Sale Price Test Restriction status.";

    ['0'] NoPriceTest 
        "No price test in place.",
    ['1'] PriceDrop 
        "Restriction in effect due to an intra-day price drop in security.",
    ['2'] RemainInEffect 
        "Restriction remains in effect.",
}


/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketParticipantPosition {
    // TODO pub mpid: Mpid (string4)
    // TODO pub stock: StockSymbol,
    ///// If `true`, firm qualifies as a Primary Market Maker.
    // TODO pub is_primary_market_maker: bool,
    pub mode: MarketMakerMode,
    pub state: MarketParticipantState,

}

impl MarketParticipantPosition {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;
        let (input, mode) = MarketMakerMode::parse(input)?;
        let (input, state) = MarketParticipantState::parse(input)?;

        Ok((input, Self { /*stock,*/ mode, state }))
    }

}

crate::types::define_enum!{

    MarketMakerMode:
        "Quoting participant’s registration status in relation to SEC Rules 101 
        and 104 of Regulation M.";

    ['N'] Normal
        "",
    ['P'] Passive
        "",
    ['S'] Syndicate
        "",
    ['R'] PreSyndicate
        "",
    ['L'] Penalty
        "",
}

crate::types::define_enum!{

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


/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MwcbDeclineLevel {
    // TODO pub level_1: Price64
    // TODO pub level_2: Price64
    // TODO pub level_3: Price64
}

impl MwcbDeclineLevel {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /**/ }))
    }

}


/// Sent when a Market-wide Circuit Breaker (MWCB) has breached a level.
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

crate::types::define_enum!{

    BreachedLevel:
        "Denotes the MWCB Level that was breached.";

    ['1'] _1
        "Level 1",
    ['2'] _2
        "Level 2",
    ['3'] _3
        "Level 3",
}


/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuotingPeriodUpdate {
    // TODO pub release_time: seconds since midnight
    pub qualifier: IpoQuotationReleaseQualifier,
    // TODO pub ipo_price: Price4,
}

impl QuotingPeriodUpdate {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        let (input, qualifier) = IpoQuotationReleaseQualifier::parse(input)?;

        Ok((input, Self { qualifier }))
    }

}

crate::types::define_enum!{

    IpoQuotationReleaseQualifier:
        "";

    ['A'] Anticipated
        "Nasdaq Market Operations has entered the IPO instrument for release",
    ['C'] Canceled
        "Nasdaq Market Operations has canceled or postponed the IPO release.",
}


/// Auction collar thresholds within which a paused security can reopen 
/// following a Limit-Up/Limit-down (LULD) Trading Pause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LuldAuctionCollar {
    // TODO pub stock: StockSymbol,
    // TODO pub reference_price: Price32,
    // TODO pub upper_price: Price32,
    // TODO pub lower_price: Price32,
    /// Indicates the number of the extensions to the Reopening Auction.
    pub extension: u32,
}

impl LuldAuctionCollar {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;
        let (input, extension) = be_u32(input)?;

        Ok((input, Self {
            // stock,
            // reference_price,
            // upper_price,
            // lower_price,
            extension
        }))
    }
}


/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalHalt {
    // TODO pub stock: StockSymbol,
    pub market: MarketCode,
    pub action: HaltAction,
}

impl OperationalHalt {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;
        let (input, market) = MarketCode::parse(input)?;
        let (input, action) = HaltAction::parse(input)?;

        Ok((input, Self { /*stock,*/ market, action }))
    }
}

crate::types::define_enum!{

    MarketCode:
        "Market being halted:";

    ['Q'] Nasdaq
        "NASDAQ",
    ['B'] Bx
        "BX",
    ['X'] Psx
        "PSX"
}

crate::types::define_enum!{

    HaltAction:
        "Market halt status:";

    ['H'] Halted
        "Operationally Halted on the identified Market.",
    ['T'] Trading
        "Operational Halt has been lifted and Trading resumed.",
}

