
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
        "Current trading state of the stock";

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
