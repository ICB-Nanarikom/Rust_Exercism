use std::collections::HashMap;

struct Tournament(HashMap<String, [i32; 3]>);

impl Tournament {
    pub fn new() -> Self {
        Tournament(HashMap::new())
    }

    fn add(&mut self, team: &str, op: usize) {
        match self.0.get_mut(team) {
            Some(v) => {
                v[op] += 1;
            },
            None => {
                let mut tmp = [0, 0, 0];
                tmp[op] += 1;
                self.0.insert(team.to_string(), tmp);
            },
        }
    }

    pub fn add_match(&mut self, match_result: &str) {
        if match_result == "" {
            return;
        }
        
        let mut it = match_result.split(';');
        let u = it.next().unwrap();
        let v = it.next().unwrap();
        let op = match it.next().unwrap() {
            "win" => 0,
            "draw" => 1,
            _ /* "loss" */ => 2,
        } as usize;
        self.add(u, op);
        self.add(v, 2 - op);
    }

    pub fn to_string(&self) -> String {
        let mut list = Vec::new();
        for (k, v) in self.0.iter() {
            list.push((
                k.clone(),
                v[0] + v[1] + v[2],
                v[0],
                v[1],
                v[2],
                3 * v[0] + v[1]
            ));
        }
        list.sort_by(|a, b| if a.5 != b.5 { b.5.cmp(&a.5) } else { a.0.cmp(&b.0) } );

        let mut ret = String::from("Team                           | MP |  W |  D |  L |  P");
        for (mut name, wp, w, d, l, score) in list {
            name += " ".repeat(31).as_str();
            name.truncate(31);
            print!("{name}");
            ret.push_str(format!("\n{name}|  {wp} |  {w} |  {d} |  {l} |  {score}").as_str());
        }
        ret
    }
}
  
pub fn tally(match_results: &str) -> String {
    let mut tournament = Tournament::new();
    match_results.split("\n").map(|match_result| tournament.add_match(match_result) ).for_each(drop);
    tournament.to_string()
}
