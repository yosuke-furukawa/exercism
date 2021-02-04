#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: String
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut b = [0; 1];
        dna.chars().enumerate().try_fold(Dna{dna: String::new()}, |dna, (i, c)| {
            if c != 'G' && c != 'C' && c != 'A' && c != 'T' {
                return Err(i);
            }
            let mut d = dna.dna;
            d += c.encode_utf8(&mut b);
            Ok(Dna{dna: d})
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna = self.dna.chars().map(|c| {
            match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!("no such chars"),
            }
        }).collect();
        Rna{ rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut b = [0; 1];
        rna.chars().enumerate().try_fold(Rna{rna: String::new()}, |rna, (i, c)| {
            if c != 'C' && c != 'G' && c != 'A' && c != 'U' {
                return Err(i);
            }
            let mut r = rna.rna;
            r += c.encode_utf8(&mut b);
            Ok(Rna{rna: r})
        })
    }
}
