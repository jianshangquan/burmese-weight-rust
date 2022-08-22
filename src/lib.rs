mod burmese_gold_weight;
mod si_weight;

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use crate::burmese_gold_weight::BurmeseGoldWeight;
    use crate::si_weight::SIWeight;

    #[test]
    fn it_works() {
        // let from_gram: BurmeseGoldWeight = BurmeseGoldWeight::fromGram(12465);
        // let from_gram2: BurmeseGoldWeight = BurmeseGoldWeight::from_siweight(&SIWeight::new(123465));

        let ring_with_copper: BurmeseGoldWeight = BurmeseGoldWeight::new(0, 1, 0, 0.0);
        let wastage: BurmeseGoldWeight = BurmeseGoldWeight::new(0,0,1,2.0);
        // let copper: BurmeseGoldWeight = BurmeseGoldWeight::new(0,0,1,0.0);

        // let purified_gold: BurmeseGoldWeight = ring_with_copper.by_burmese_gold_quality(15);
        // let purified_gold2: BurmeseGoldWeight = ring_with_copper.by_international_gold_quality(22);


        let gold: BurmeseGoldWeight = ring_with_copper.add(&wastage);
        // let pureGold: BurmeseGoldWeight = ring_with_copper.substract(&copper);


        // let sell: BigDecimal = ring_with_copper.by_burmese_market_value_price(
        //     3_000_000,
        //     50_000
        // );

        let result : BurmeseGoldWeight;

        // ring_with_copper.to_patetha();
        // ring_with_copper.to_kyat();
        // ring_with_copper.to_pae();
        // ring_with_copper.to_yway();
        // ring_with_copper.to_gram();

        result = gold;
        println!("{}", result);

        // assert_eq!(result, 4);
    }
}
