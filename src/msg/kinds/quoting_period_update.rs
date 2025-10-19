
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
