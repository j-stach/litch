
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

crate::types::define_enum!{

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

crate::types::define_enum!{

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

crate::types::define_enum!{

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
