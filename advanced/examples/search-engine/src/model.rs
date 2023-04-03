pub struct Boolean<'a> {
    query_type: &'a str,
}

impl<'a> Boolean<'a> {
    pub fn new() -> Self {
        Self { query_type: "AND" }
    }
    pub fn retrieve<'b, 'c>(query: &'b str) -> Vec<&'c str> {}

    fn intersect_increasing_freq() {}
    fn intersect() {}
}
