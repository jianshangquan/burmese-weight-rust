# burmese-gold-weight-rust
### Unit Conversion table
|From|To|
|--|--|
|၁ ပိဿာ| ၁၀၀ကျပ်|
|၁ ကျပ်| ၁၆ပဲ|
|၁ ကျပ်| ၁၆.၆၆၆၆၆၆၆၆ ဂရမ်|
|၁ ပဲ| ၈ရွေး|
|၁ ပဲ| ၁.၀၄၁၆၆၆၆၆၆၂၅ ဂရမ်|

> Source: Yangon Gold Market

#### Notes
1. BurmeseGoldWeight is immutable
2. Use BigDecimal for calculation due to [Floating point error in computing](https://betterprogramming.pub/why-is-0-1-0-2-not-equal-to-0-3-in-most-programming-languages-99432310d476)

#### Usage
```rust
    // Usage

    // From gram
    let from_gram: BurmeseGoldWeight = BurmeseGoldWeight::fromGram(12465);
    let from_gram2: BurmeseGoldWeight = BurmeseGoldWeight::from_siweight(&SIWeight::new(123465));
    
    // Use directly use with new
    let ring_with_copper: BurmeseGoldWeight = BurmeseGoldWeight::new(0, 1, 0, 0.0); // ရွှေထည် ၁ကျပ်သား
    let wastage: BurmeseGoldWeight = BurmeseGoldWeight::new(0,0,1,2.0); // အလျေ့ာအတွက် ၁ပဲ ၂ရွေး
    let copper: BurmeseGoldWeight = BurmeseGoldWeight::new(0,0,1,0.0); // ကြေး(အတွင်းစပ်) ၁ပဲ
    
    let purified_gold: BurmeseGoldWeight = ring_with_copper.by_burmese_gold_quality(15); // ၁၅ပဲရည် အခေါက်ရွှေချွတ်ပြီး
    let purified_gold2: BurmeseGoldWeight = ring_with_copper.by_international_gold_quality(22); // 22/24 K အခေါက်ရွှေချွတ်ပြီး
    
    
    let gold: BurmeseGoldWeight = ring_with_copper.add(&wastage); // ရွှေထည် + အလျော့အတွက် = အထည်လုပ် အချိန်
    let pureGold: BurmeseGoldWeight = ring_with_copper.substract(&copper); // ရွှေထည် - ကြေး = အခေါက်
    
    // ရောင်းစျေး/ ဝယ်စျေး
    let sell: BigDecimal = ring_with_copper.by_burmese_market_value_price(
        3_000_000, // အခေါက်ရွှေစျေး
        50_000 // ခွာစျေး
    ); // အခေါက်ရွှေ သိန်း၃၀ ပေါက်စျေး၏ ရွှေထည်ရောင်းစျေး
    
    
    ring_with_copper.to_patetha(); // စုစုပေါင်း ပိဿာအချိန်         => 0.01
    ring_with_copper.to_kyat(); // စုစုပေါင်း ကျပ်အချိန်            => 1
    ring_with_copper.to_pae(); // စုစုပေါင်း ပဲအချိန်               => 16
    ring_with_copper.to_yway(); // စုစုပေါင်း ရွှေးအချိန်            => 128 
    ring_with_copper.to_gram(); // စုစုပေါင်း gram ဂရမ်အချိန်       => 16.66666666
```


### Ported languages
Java [burmese-gold-weight-java](https://github.com/jianshangquan/burmese-gold-weight-java) \
Javascript [burmese-gold-weight-javascript](https://github.com/jianshangquan/burmese-weight-js) \
Dart  **Comming soon** \
Rust [burmese-gold-weight-rust](https://github.com/jianshangquan/burmese-weight-rust)
