struct Score {
    score: i32,
}

impl Score {
    fn get_grade(&self) -> String {
        if self.score >= 90 {
            String::from("A")
        } else if self.score >= 80 {
            String::from("B")
        } else {
            String::from("C")
        }
    }

    fn from(score: i32) -> Score {
        Score { score: score }
    }
}

fn main() {

}

#[test]
fn test_get_grade() {
    assert_eq!(Score::from(100).get_grade(), "A");
    assert_eq!(Score::from(80).get_grade(), "B");
}