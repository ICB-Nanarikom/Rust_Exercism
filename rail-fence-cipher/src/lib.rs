pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails: rails }
    }

    fn getpos(&self, len: u32) -> Vec<[u32; 2]> {
        let mut r = (1..=self.rails).chain((2..self.rails).rev()).cycle();
        (1..=len).map(|i| [r.next().unwrap(), i]).collect::<Vec<_>>()
    }

    pub fn encode(&self, text: &str) -> String {
        let mut pos = self.getpos(text.len() as u32);
        pos.sort_by(|[r1, c1], [r2, c2]| if c1 != c2 { c1.cmp(c2) } else { r1.cmp(r2) } );
        let mut it = pos.iter();
        let mut v = text.chars().map(|ch| (it.next().unwrap(), ch)).collect::<Vec<_>>();
        v.sort_by(|([r1, c1], _), ([r2, c2], _)| if r1 != r2 { r1.cmp(r2) } else { c1.cmp(c2) } );
        v.into_iter().map(|(_, ch)| ch).collect::<String>()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut pos = self.getpos(cipher.len() as u32);
        pos.sort_by(|[r1, c1], [r2, c2]| if r1 != r2 { r1.cmp(r2) } else { c1.cmp(c2) } );
        let mut it = pos.iter();
        let mut v = cipher.chars().map(|ch| (it.next().unwrap(), ch)).collect::<Vec<_>>();
        v.sort_by(|([r1, c1], _), ([r2, c2], _)| if c1 != c2 { c1.cmp(c2) } else { r1.cmp(r2) } );
        v.into_iter().map(|(_, ch)| ch).collect::<String>()
    }
}
