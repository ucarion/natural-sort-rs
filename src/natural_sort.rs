#[deriving(Show, PartialEq, Eq)]
enum StringElem<'a> {
    Letters(&'a str),
    Number(int)
}

#[deriving(Show, PartialEq, Eq)]
struct NumberSequence<'a> {
    elems: Vec<StringElem<'a>>
}

impl<'a> NumberSequence<'a> {
    fn from_str(string: &'a str) -> NumberSequence<'a> {
        let numbers_re = regex!(r"^\p{N}+");
        let mut elems: Vec<StringElem> = Vec::new();
        let mut to_parse = String::from_str(string);

        while !to_parse.is_empty() {
            match numbers_re.find(string) {
                Some((_, end_index)) => {
                    let prefix_to_num: int =
                        from_str(to_parse.slice_to(end_index)).unwrap();
                    elems.push(StringElem::Number(prefix_to_num));

                    to_parse = to_parse.slice_from(end_index).to_string();
                },
                None => {}
            };
        }

        NumberSequence { elems: elems }
    }
}

#[test]
fn test_makes_numseq() {
    let str1 = "123";
    let seq1 = NumberSequence { elems: vec![StringElem::Number(123)] };
    assert_eq!(NumberSequence::from_str(str1), seq1);
}
