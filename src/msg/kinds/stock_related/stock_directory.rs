
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
        "Indicates if the Nasdaq security is set up for IPO release. 
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

