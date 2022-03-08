use crate::Vector;

    #[test]
    fn add() {
        let mut v = Vector::new();
    v.add(9);
    assert_eq!(v.valoare,vec![9]);
    }