use helix_dev_doc_deck::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 69, slack: 51, drag: 29, confidence: 75 };
    assert_eq!(review_score(case), 177);
    assert_eq!(review_lane(case), "ship");
}
