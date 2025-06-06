#[derive(Debug, Clone, Copy)]
struct BinSeq<'a> {
    len: usize,
    content: &'a [bool],
}

impl <'a> From<String> for BinSeq<'a> {
    fn from(s: String) -> Self {
        let bits: String = s.into_bytes()
            .iter()
            .map(|&c| format!("{c:08b}"))
            .collect();
        bits.chars();
        println!("Bits: {:?}", bits);
        Self {len: 0, content: &[]}
    }
}

impl <'a> BinSeq<'a> {
    fn compress(&self) -> BinSeq {
        todo!();
    }
}

fn main() {
    let b = BinSeq::from("日本語 ENG €".to_string());
    let comp= b.compress();
    dbg!(b);
}