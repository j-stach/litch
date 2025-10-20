
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
