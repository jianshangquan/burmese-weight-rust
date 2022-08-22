use std::{ops::{Mul, Div, Add, Rem, Sub}, str::FromStr};

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};




struct BurmeseGoldWeight{
    patetha: BigDecimal,
    kyat: BigDecimal,
    pae: BigDecimal,
    yway: BigDecimal,
}



impl BurmeseGoldWeight{

    const ONE_PATETHA_IN_KYAT: BigDecimal = BigDecimal::from_str("100").unwrap();
    const ONE_KYAT_IN_PAE: BigDecimal = BigDecimal::from_str("16").unwrap();
    const ONE_PAE_IN_YWAY: BigDecimal = BigDecimal::from_str("8").unwrap();
    const ONE_KYAT_IN_GRAM: BigDecimal = BigDecimal::from_str("16.66666666").unwrap();
    const ONE_PAE_IN_GRAM: BigDecimal = BurmeseGoldWeight::ONE_KYAT_IN_GRAM.div(BurmeseGoldWeight::ONE_KYAT_IN_PAE);

    pub fn new(){
        
    }

    pub fn fromGram(&mut self, gram: u128){
        let w: BurmeseGoldWeight = self.from_gram(gram);
        self.patetha = w.patetha;
        self.kyat = w.kyat;
        self.pae = w.pae;
        self.yway = w.yway;
    }

    pub fn fromSIWeight(&self){

    }

    pub fn clone(&self) -> BurmeseGoldWeight{
        return BurmeseGoldWeight {
            patetha: self.patetha.clone(),
            kyat: self.kyat.clone(),
            pae: self.pae.clone(),
            yway: self.yway.clone()
        }
    }


    pub fn byBurmeseGoldQuality(&self, quality: u8) -> BurmeseGoldWeight{
        let ratio: BigDecimal = BigDecimal::from_u8(quality).unwrap().div(BigDecimal::from_u8(16).unwrap());
        let purifiedK: BigDecimal = self.toKyat().mul(ratio);
        return self.fromKyatToBurmeseGoldWeight(&purifiedK);
    }

    pub fn byInternationalGoldQuality(&self, quality: u8) -> BurmeseGoldWeight{
        let ratio: BigDecimal = BigDecimal::from_u8(quality).unwrap().div(BigDecimal::from_u8(24).unwrap()).mul(BigDecimal::from_u8(16).unwrap());
        let purifiedK: BigDecimal = self.toKyat().mul(ratio);
        return self.fromKyatToBurmeseGoldWeight(&purifiedK);
    }


    pub fn add(&self, weight: BurmeseGoldWeight) -> BurmeseGoldWeight{
        let k: BigDecimal = weight.toKyat().add(self.toKyat());
        return self.fromKyatToBurmeseGoldWeight(&k);
    }

    pub fn substract(&self, weight: BurmeseGoldWeight) -> BurmeseGoldWeight{
        let k: BigDecimal = weight.toKyat().sub(self.toKyat());
        return self.fromKyatToBurmeseGoldWeight(&k);
    }

    pub fn toPatetha(&self) -> BigDecimal{
        return self.toKyat().div(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT);
    }

    pub fn toKyat(&self) -> BigDecimal{
        return self.toPae().div(BurmeseGoldWeight::ONE_KYAT_IN_PAE);
    }

    pub fn toPae(&self) -> BigDecimal{
        let result: BigDecimal = self.patetha.mul(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT).mul(BurmeseGoldWeight::ONE_KYAT_IN_PAE).add(self.kyat.mul(BurmeseGoldWeight::ONE_KYAT_IN_PAE)).add(&self.pae).add(self.yway.div(BurmeseGoldWeight::ONE_PAE_IN_YWAY));
        return result;
    }

    pub fn toYway(&self) -> BigDecimal{
        return self.toPae().mul(BurmeseGoldWeight::ONE_PAE_IN_YWAY);
    }


    fn fromKyatToBurmeseGoldWeight(&self, kyat: &BigDecimal) -> BurmeseGoldWeight{
        let mut pa: BigDecimal = BigDecimal::default();
        let mut k: BigDecimal = BigDecimal::default();
        let mut p: BigDecimal = BigDecimal::default();
        let mut y: BigDecimal = BigDecimal::default();

        let modulo: BigDecimal = BigDecimal::from_str("1").unwrap();

        pa = kyat.div(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT);
        pa = pa.sub(pa.rem(&modulo));

        k = kyat.sub(&pa);
        k = k.sub(k.rem(&modulo));

        p = kyat.sub(pa.mul(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT)).sub(&k).div(BurmeseGoldWeight::ONE_KYAT_IN_PAE);
        p = p.sub(p.rem(modulo));

        y = kyat.sub(pa.mul(BurmeseGoldWeight::ONE_PATETHA_IN_KYAT)).sub(&k).sub(p.div(BurmeseGoldWeight::ONE_KYAT_IN_PAE));

        return BurmeseGoldWeight { patetha: pa, kyat: k, pae: p, yway: y };
    }

    fn from_gram(&self, g: u128) -> BurmeseGoldWeight{
        let k: BigDecimal = BigDecimal::from_u128(g).unwrap();
        return self.fromKyatToBurmeseGoldWeight(&k);
    }
}

