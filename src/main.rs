use std::env;
use std::io;
use std::fs;
use std::io::BufRead;


struct LSubGroup {
    l_last: u8,
    inp: Vec<Vec<u8>>,
    outp: Vec<u8>,
    counts: Vec<u8>,
    n_match: u8,
    i_next: u8,
    lim: u16,
    eph: u8
}


impl LSubGroup {

    fn info(&self) {
        println!("Info: LL: {:?}, Out: {:?}, Counts: {:?}, Match Count: {:?}, Next Index: {:?}, Current: {:?}", 
            char::from(self.l_last),
            String::from_utf8(self.outp.clone()).unwrap(),
            self.counts,
            self.n_match,
            self.i_next,
            String::from_utf8(self.inp[0].clone()).unwrap()
        );
    }
    
    fn decount(&mut self) {
        for x in self.counts.iter_mut() {
            *x -= 1;
        }
    }

    fn zero(&mut self) {
        self.n_match = 0;
        self.i_next = 0;
    }

    fn incr(&mut self) {
        self.i_next += 1;
        self.n_match += 1;
    }

    fn consecutive_match(&mut self) {
        if Some(&self.n_match) == self.counts.last() && self.outp.last() == Some(&40u8) {
            self.outp.pop();
        } else {
            self.counts.push(self.n_match);   
        }
        self.outp.extend([self.l_last, 40u8].iter());
        Self::zero(self);
    }

}

fn extract_sub_group(master_list: &mut Vec<Vec<u8>>) -> LSubGroup {

    let first_letter: u8 = master_list[0][0];
    let mut in_vec: Vec<Vec<u8>> = Vec::new();
    let out_vec: Vec<u8> = Vec::from([first_letter, 40u8]);

    let mut temp: Vec<u8>;
    while master_list.len() > 0 && master_list[0][0] == first_letter {
        temp = master_list.remove(0);
        temp.remove(0);
        in_vec.push(temp);
    }

    let sub_group = LSubGroup {
        l_last: first_letter,
        inp: in_vec.to_vec(),
        outp: out_vec,
        counts: Vec::from([in_vec.len() as u8]),
        n_match: 0,
        i_next: 0,
        lim: 1024,
        eph: 0
    };

    return sub_group;
}


fn treeify(mut words: LSubGroup, dbg: bool) -> Option<Vec<u8>> {

    if words.inp.len() == 1 {
        words.outp.pop();
        words.outp.append(&mut words.inp[0]);
        words.outp.push(124u8);
        return Some(words.outp)
    }

    while words.inp.len() > 0 {
        if dbg { words.info() };

        if words.inp.len() == 1 {
            // Last Line
            words.outp.append(&mut words.inp.remove(0));
            for _ in 0..words.counts.len() {
                words.outp.push(41u8);
            }
            words.outp.push(124u8);

        } else if words.i_next == 0 {
            // Index reset to 0
            if words.inp[0].len() == 0 {
                words.inp.remove(0);
                words.l_last = 0;
                words.eph += 1;
                words.i_next = 0;
                words.decount();
            } else {
                words.l_last = words.inp[0].remove(0);
                words.incr()
            }
        } else if words.l_last == words.inp[words.i_next as usize][0] {
            // Letter Matched
            if &words.i_next >= words.counts.last().unwrap() {
                words.outp.push(words.l_last);
                words.zero();
            } else {
                words.inp[words.i_next as usize].remove(0);
                words.incr();
            }
            if words.i_next >= words.inp.len() as u8 {
                words.consecutive_match();
            }
        } else if words.l_last != words.inp[words.i_next as usize][0] {
            // Letter not Matched
            if words.n_match > 1 {
                words.consecutive_match();
            } else if words.n_match == 1 {
                if words.inp[0].len() == 0 && words.eph > 0 && words.counts.last() == Some(&1) {
                    words.inp.remove(0);
                    words.outp.pop();
                    words.counts.pop();
                    words.outp.push(words.l_last);
                    words.outp.push(63u8);
                    words.eph -= 1;
                } else {
                    words.outp.push(words.l_last);
                    words.outp.append(&mut words.inp.remove(0));
                }
                if words.counts.last() > Some(&1u8) {
                    words.outp.push(124u8);
                }
                words.decount();
                words.zero();
            }
        }

        while words.counts.last() < Some(&1) {
            // Clear Counts
            words.outp.push(41u8);
            if words.eph > 0 {
                words.outp.push(63u8);
                words.eph -= 1;
            }
            if words.counts.len() > 1 {
                words.counts.reverse();
                if words.counts[1] != 0 {
                    words.outp.push(124u8);
                }
                words.counts.reverse();
            }
            words.counts.pop();
        }

        words.lim -= 1;
        if words.lim == 0 {
            return None
        }
    }
    return Some(words.outp)
}

fn get_file(file_name: String) -> io::Result<Vec<Vec<u8>>> {
    let word_vectors: Vec<Vec<u8>> = io::Cursor::new(fs::read(&file_name)?).split(b'\n').map(|x| x.unwrap()).collect();
    return Ok(word_vectors)
}

fn clean(input: &mut Vec<Vec<u8>>) {

    for x in input.iter_mut() {
        while x.len() > 0 && x.last() == Some(&32) {
            x.pop();
        }
        while x.len() > 0 && x[0] == 32 {
            x.remove(0);
        }
    }
    input.retain(|x| x.len() > 0);
    input.sort();
    input.dedup();
}

fn parse_file(file: String, dbg: bool) -> Vec<u8> {

    let mut wlist: Vec<Vec<u8>> = get_file(file).unwrap();
    let mut word_tree: Vec<u8> = Vec::new();
    let mut letter_group: LSubGroup;
    let mut sub_tree: Vec<u8>;
    
    clean(&mut wlist);
    while wlist.len() > 0 {
        letter_group = extract_sub_group(&mut wlist);
        sub_tree = treeify(letter_group, dbg).unwrap();
        word_tree.append(&mut sub_tree);
    }

    word_tree.pop();
    return word_tree
}

fn main() -> io::Result<()> {

    let argv: Vec<String> = env::args().collect();
    let output: Vec<u8> = parse_file(argv[1].clone(), false);

    println!("{}", String::from_utf8(output.clone()).unwrap());
    Ok(())
}




#[cfg(test)]
mod tests {
    use crate::parse_file;

    #[test]
    fn overall_1() {
        let locus: String = String::from("tests/tfile1");
        assert_eq!(
            "A(aron|b(dullah|igail)|dam|hmed|l(an|bert|e(ssandro|x(ander|is))|i(ce)?|ma)|m(anda|ber|elia|y)|n(astasia|dre(a|w)|gela|na?|t(hony|oni))|rthur|shley|u(rora|stin)|va)",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );        
    }

    #[test]
    fn overall_2() {
        let locus: String = String::from("tests/tfile2");
        assert_eq!(
            "S(a(m(antha|uel)|ndra|rah?)|cott|e(an|rgei)|h(aron|irley)|o(f(ia|ía)|phia)|te(ph(anie|en)|ven)|usan)",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }
    
    #[test]
    fn overall_3() {
        let locus: String = String::from("tests/tfile3");
        assert_eq!(
            "M(a(dison|hmoud|r(garet|i(a|e|lyn)|k|t(ha|ina|ín)|y(am)?|ía)|son|t(eo|t(eo|hew))|xim)|e(gan|lissa)|i(ch(ael|elle)|khail)|ohamed|ustafa)",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }
    
    #[test]
    fn overall_4() {
        let locus: String = String::from("tests/tfile4");
        assert_eq!(
            "L(eon(ardo)?|i(am|nda|sa)|o(gan|r(enzo|i)|uise?)|ucía|yn|éo)",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }

    #[test]
    fn overall_5() {
        let locus: String = String::from("tests/tfile5");
        assert_eq!(
            "E(mma|ric|than|ugene|velyn)|F(atima|ran(ces(co)?|k))|O(liv(er|ia)|mar)|P(a(mela|tric(ia|k)|ul)|eter)|W(ayne|illi(am|e))|Y(elena|ousouf)|Zachary",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }

    #[test]
    fn single_line_groups() {
        let locus: String = String::from("tests/tfile6");
        assert_eq!(
            "abc|bcd|efg",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }

    #[test]
    fn excess_optionals() {
        let locus: String = String::from("tests/tfile7");
        assert_eq!(
            "a(bc(d(dd)?|e|fff)?|zzz)",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }

    #[test]
    fn out_of_order() {
        let locus: String = String::from("tests/tfile8");
        assert_eq!(
            "E(mma|ric|than|ugene|velyn)|F(atima|ran(ces(co)?|k))|O(liv(er|ia)|mar)|P(a(mela|tric(ia|k)|ul)|eter)|W(ayne|illi(am|e))|Y(elena|ousouf)|Zachary",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }

    #[test]
    fn empty_lines() {
        let locus: String = String::from("tests/tfile9");
        assert_eq!(
            "abc|cde|fgh",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }

    #[test]
    fn leading_trailing_whitespace() {
        let locus: String = String::from("tests/tfile10");
        assert_eq!(
            "a(bc|cd)|cde|fg(h|y)",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }

    #[test]
    fn non_unique_lines() {
        let locus: String = String::from("tests/tfile11");
        assert_eq!(
            "a(bc|cd)|cde|fg(h|y)",
            String::from_utf8(parse_file(locus, false).clone()).unwrap()
        );
    }

}











