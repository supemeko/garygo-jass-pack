#[cfg(test)]
mod test2ualu {
    enum Op {
        Add(usize, usize),
        Sub(usize, usize),
    }

    fn foo(op: fn(usize, usize) -> Op) {
        let op2: fn(usize, usize) -> Op = Op::Add;
        assert!(op == op2);
    }

    #[test]
    fn it_not_works() {
        let op: fn(usize, usize) -> Op = Op::Add;
        foo(op)
    }
}
