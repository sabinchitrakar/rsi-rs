
#![feature(external_doc)]
use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;
use ta_common::helpers::trend;
use wilders_rs::Wilders;
#[doc(include = "../README.md")]
pub struct RSI {
    wilders_up: Wilders,
    wilders_down: Wilders,
    period: u32,
    history: FixedQueue<f64>,
    prev: Option<f64>,

}

impl RSI {
    pub fn new(period: u32) -> RSI {
        Self {
            period,
            wilders_up: Wilders::new(period),
            wilders_down: Wilders::new(period),
            history: FixedQueue::new(period),
            prev: None,

        }
    }
}

impl Indicator<f64, Option<f64>> for RSI {
    fn next(&mut self, input: f64) -> Option<f64> {
        let res = match self.prev {
            None => None,
            Some(prev) => {
                let [up, down] = trend(prev, input);
                let avg_up = self.wilders_up.next(up);
                let avg_down = self.wilders_down.next(down);
                if self.history.is_full() {
                    let rs = avg_up.unwrap() / avg_down.unwrap();
                    let rsi = 100.0 - (100.0 / (1.0 + rs));
                    return Some(rsi);
                }
                None
            }
        };
        self.prev = Some(input);
        self.history.add(input);
        res
    }

    fn reset(&mut self) {
        self.wilders_down.reset();
        self.history.clear();
        self.wilders_up.reset();
        self.prev = None;
    }
}

#[cfg(test)]
mod tests {
    use crate::RSI;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
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
    }
}
