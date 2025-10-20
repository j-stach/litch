
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchTrade {
    // pub order_ref_num: u64,
    // pub side: Side, NOTE: This will always be "B". can probably just ignore
    // pub quantity: u32,
    // TODO pub stock: StockSymbol,
    // pub price: Price32,
    // pub match_number: u64,
}

impl MatchTrade {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;

        Ok((input, Self { /*stock,*/ }))
    }

}


/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrossTrade {
    // pub order_ref_num: u64,
    // pub quantity: u32,
    // TODO pub stock: StockSymbol,
    // pub price: Price32,
    // pub match_number: u64,
    pub cross_type: CrossType,
}

impl CrossTrade {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO
        // let (input, stock) = StockSymbol::parse(input)?;
        let (input, cross_type) = CrossType::parse(input)?;

        Ok((input, Self { /*stock,*/ cross_type }))
    }

}


/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrokenTrade {
    // pub match_number: u64,
}

impl BrokenTrade {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO

        Ok((input, Self { /*stock,*/ }))
    }

}


crate::types::define_enum!{

    CrossType:
        "";

    ['O'] Opening
        "",
    ['C'] Closing
        "",
    ['H'] Halt
        "",
}


