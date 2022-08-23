use std::{ops::{Mul, Div, Add, Rem, Sub}, str::FromStr};
use std::fmt::{Display, Formatter};

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use crate::si_weight::SIWeight;


pub struct BurmeseGoldWeight{
    patetha: BigDecimal,
    kyat: BigDecimal,
    pae: BigDecimal,
    yway: BigDecimal,
}



impl BurmeseGoldWeight{

    // let ONE_PATETHA_IN_KYAT: BigDecimal = BigDecimal::from_str("100").unwrap();
    // let ONE_KYAT_IN_PAE: BigDecimal = BigDecimal::from_str("16").unwrap();
    // let ONE_PAE_IN_YWAY: BigDecimal = BigDecimal::from_str("8").unwrap();
    // let ONE_KYAT_IN_GRAM: BigDecimal = BigDecimal::from_str("16.66666666").unwrap();
    // let ONE_PAE_IN_GRAM: BigDecimal = BurmeseGoldWeight::ONE_KYAT_IN_GRAM.div(BurmeseGoldWeight::ONE_KYAT_IN_PAE);


    // static
    fn ONE_PATETHA_IN_KYAT() -> BigDecimal{
        BigDecimal::from_str("100").unwrap()
    }

    fn ONE_KYAT_IN_PAE() -> BigDecimal{
        BigDecimal::from_str("16").unwrap()
    }

    fn ONE_PAE_IN_YWAY() -> BigDecimal{
        BigDecimal::from_str("8").unwrap()
    }

    fn ONE_KYAT_IN_GRAM() -> BigDecimal{
        BigDecimal::from_str("16.66666666").unwrap()
    }
    fn ONE_KYAT_IN_YWAY() -> BigDecimal{
        BigDecimal::from_str("128").unwrap()
    }

    fn ONE_PAE_IN_GRAM() -> BigDecimal{
        // BurmeseGoldWeight::ONE_KYAT_IN_GRAM().div(BurmeseGoldWeight::ONE_KYAT_IN_PAE)
        BurmeseGoldWeight::ONE_KYAT_IN_GRAM().div(BurmeseGoldWeight::ONE_KYAT_IN_PAE())
    }



    pub fn new(patetha: i128, kyat: i8, pae: i8, yway: f32) -> BurmeseGoldWeight{
        return BurmeseGoldWeight{
            patetha: BigDecimal::from_i128(patetha).unwrap(),
            kyat: BigDecimal::from_i8(kyat).unwrap(),
            pae: BigDecimal::from_i8(pae).unwrap(),
            yway: BigDecimal::from_f32(yway).unwrap()
        }
    }

    pub fn fromGram(gram: u128) -> BurmeseGoldWeight{
        BurmeseGoldWeight::from_gram(gram)
    }

    pub fn from_siweight(siWeight: &SIWeight) -> BurmeseGoldWeight{
        BurmeseGoldWeight::fromGram(siWeight.getGram().to_u128().unwrap())
    }

    pub fn clone(&self) -> BurmeseGoldWeight{
        return BurmeseGoldWeight {
            patetha: self.patetha.clone(),
            kyat: self.kyat.clone(),
            pae: self.pae.clone(),
            yway: self.yway.clone()
        }
    }


    pub fn by_burmese_gold_quality(&self, quality: u8) -> BurmeseGoldWeight{
        let ratio: BigDecimal = BigDecimal::from_u8(quality).unwrap().div(BigDecimal::from_u8(16).unwrap());
        let purifiedK: BigDecimal = self.to_kyat().mul(ratio);
        return BurmeseGoldWeight::from_kyat_to_burmese_gold_weight(&purifiedK);
    }

    pub fn by_international_gold_quality(&self, quality: u8) -> BurmeseGoldWeight{
        let ratio: BigDecimal = BigDecimal::from_u8(quality).unwrap().div(BigDecimal::from_u8(24).unwrap()).mul(BigDecimal::from_u8(16).unwrap());
        let purifiedK: BigDecimal = self.to_kyat().mul(ratio);
        return BurmeseGoldWeight::from_kyat_to_burmese_gold_weight(&purifiedK);
    }


    pub fn add(&self, weight: &BurmeseGoldWeight) -> BurmeseGoldWeight{
        let k: BigDecimal = weight.to_kyat().add(self.to_kyat());
        // println!("k {}", k);
        return BurmeseGoldWeight::from_kyat_to_burmese_gold_weight(&k);
    }

    pub fn substract(&self, weight: &BurmeseGoldWeight) -> BurmeseGoldWeight{
        let k: BigDecimal = self.to_kyat().sub(weight.to_kyat());
        return BurmeseGoldWeight::from_kyat_to_burmese_gold_weight(&k);
    }

    pub fn to_patetha(&self) -> BigDecimal{
        return self.to_kyat().div(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT());
    }

    pub fn to_kyat(&self) -> BigDecimal{
        return self.to_pae().div(BurmeseGoldWeight::ONE_KYAT_IN_PAE());
    }

    pub fn to_pae(&self) -> BigDecimal{
        let result: BigDecimal = (&self.patetha).mul(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT()).mul(BurmeseGoldWeight::ONE_KYAT_IN_PAE()).add((&self.kyat).mul(BurmeseGoldWeight::ONE_KYAT_IN_PAE())).add(&self.pae).add((&self.yway).div(BurmeseGoldWeight::ONE_PAE_IN_YWAY()));
        return result;
    }

    pub fn to_gram(&self) -> BigDecimal{
        let p = self.to_pae();
        p.mul(BurmeseGoldWeight::ONE_PAE_IN_GRAM())
    }

    pub fn to_yway(&self) -> BigDecimal{
        return self.to_pae().mul(BurmeseGoldWeight::ONE_PAE_IN_YWAY());
    }

    pub fn by_burmese_market_value_price(&self, burmese_gold_spotprice: u128, gap: u128) -> BigDecimal{
        let spot: BigDecimal = BigDecimal::from_u128(burmese_gold_spotprice).unwrap();
        let g: BigDecimal = BigDecimal::from_u128(gap).unwrap();
        self.to_kyat().mul(spot.sub(g))
    }


    fn from_kyat_to_burmese_gold_weight(kyat: &BigDecimal) -> BurmeseGoldWeight{


        let mut pa: BigDecimal = BigDecimal::default();
        let mut k: BigDecimal = BigDecimal::default();
        let mut p: BigDecimal = BigDecimal::default();
        let mut y: BigDecimal = BigDecimal::default();

        let modulo: BigDecimal = BigDecimal::from_str("1").unwrap();

        pa = kyat.div(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT());
        let cpa = pa.clone();
        pa = pa.sub(cpa.rem(&modulo));

        k = kyat.sub(&pa);
        let ck = k.clone();
        k = k.sub(ck.rem(&modulo));

        p = kyat.sub(pa.clone().mul(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT())).sub(&k).mul(BurmeseGoldWeight::ONE_KYAT_IN_PAE());
        let cp = p.clone();
        p = p.sub(cp.rem(&modulo));

        y = kyat.sub(pa.clone().mul(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT())).sub(&k).sub(p.clone().div(BurmeseGoldWeight::ONE_KYAT_IN_PAE())).mul(BurmeseGoldWeight::ONE_KYAT_IN_YWAY());

        return BurmeseGoldWeight { patetha: pa, kyat: k, pae: p, yway: y };
        return BurmeseGoldWeight { patetha: pa, kyat: k, pae: p, yway: y };
    }

    fn from_gram(g: u128) -> BurmeseGoldWeight{
        let k: BigDecimal = BigDecimal::from_u128(g).unwrap();
        return BurmeseGoldWeight::from_kyat_to_burmese_gold_weight(&k);
    }
}


impl Display for BurmeseGoldWeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}ပိဿာ, {}ကျပ်, {}ပဲ, {}ရွေး", self.patetha.with_scale(0), self.kyat.with_scale(0), self.pae.with_scale(0), self.yway)
    }
}

