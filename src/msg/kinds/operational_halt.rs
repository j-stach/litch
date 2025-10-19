
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
        "";

    ['Q'] Nasdaq
        "NASDAQ",
    ['B'] Bx
        "BX",
    ['X'] Psx
        "PSX"
}

crate::types::define_enum!{

    HaltAction:
        "";

    ['H'] Halted
        "Operationally Halted on the identified Market.",
    ['T'] Trading
        "Operational Halt has been lifted and Trading resumed.",
}
