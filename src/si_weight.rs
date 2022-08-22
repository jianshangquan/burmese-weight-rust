use bigdecimal::{BigDecimal, FromPrimitive};

pub struct SIWeight{
    gram: BigDecimal
}


impl SIWeight {

    pub fn new(gram: u128) -> SIWeight{
        SIWeight{gram: BigDecimal::from_u128(gram).unwrap()}
    }

    pub fn newFromBigDecimal(gram: BigDecimal) -> SIWeight{
        SIWeight{gram}
    }


    pub fn getGram(&self) -> &BigDecimal{
        &self.gram
    }
}