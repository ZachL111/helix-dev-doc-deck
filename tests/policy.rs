use helix_dev_doc_deck::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 62, capacity: 81, latency: 17, risk: 25, weight: 10 };
    assert_eq!(score(signal), 19);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 62, capacity: 83, latency: 27, risk: 13, weight: 9 };
    assert_eq!(score(signal), 71);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 96, capacity: 71, latency: 26, risk: 8, weight: 4 };
    assert_eq!(score(signal), 145);
    assert_eq!(classify(signal), "review");
}
