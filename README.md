# Relative Strength Index (RSI)
```
use rsi_rs::RSI;
use ta_common::traits::Indicator;

let mut rsi = RSI::new(5);
assert_eq!(rsi.next(81.59), None);
assert_eq!(rsi.next(81.06), None);
assert_eq!(rsi.next(82.87), None);
assert_eq!(rsi.next(83.00), None);
assert_eq!(rsi.next(83.61), None);
assert_eq!(rsi.next(83.15), Some(72.03389830508483));
assert_eq!(rsi.next(82.84), Some(56.635202665186114));
assert_eq!(rsi.next(83.99), Some(61.68751532989949));
assert_eq!(rsi.next(84.55), Some(71.83428751746838));
assert_eq!(rsi.next(84.36), Some(77.71943353585428));
assert_eq!(rsi.next(85.53), Some(86.64739714536829));
assert_eq!(rsi.next(86.54), Some(92.43207672848268));
assert_eq!(rsi.next(86.89), Some(95.28836178630372));
assert_eq!(rsi.next(87.77), Some(97.05218372440096));
assert_eq!(rsi.next(87.29), Some(97.9151897656543));


```