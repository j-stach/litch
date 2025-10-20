
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectListingWithCapitalRaise {
    // stock
    // pub open_eligibility_status: bool,
    // min allowable price: Price32
    // max allowable price: Price32
    // near execution price: Price32
    // near execution time: u64
    // lower price range collar: Price32
    // upper price range collar: Price32
}

impl DirectListingWithCapitalRaise {

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        // TODO

        Ok((input, Self {  }))
    }

}

