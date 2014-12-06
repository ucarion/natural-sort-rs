#[deriving(Show, PartialEq, Eq)]
enum StringElem {
    Letters(String),
    Number(int)
}

#[deriving(Show, PartialEq, Eq)]
pub struct NumberSequence {
    elems: Vec<StringElem>
}

impl NumberSequence {
    pub fn from_str(string: &str) -> NumberSequence {
        let numbers_re = regex!(r"^\p{N}+");
        let letters_re = regex!(r"^\P{N}+");

        let mut elems = Vec::new();
        let mut to_parse = String::from_str(string);

        while !to_parse.is_empty() {
            let numbers_match = numbers_re.find(to_parse.as_slice());

            let (next_token, next_to_parse) = if numbers_match.is_some() {
                NumberSequence::process_number(
                    numbers_match.unwrap(), to_parse)
            } else {
                let letters_match = letters_re.find(to_parse.as_slice());
                NumberSequence::process_letters(
                    letters_match.unwrap(), to_parse)
            };

            elems.push(next_token);
            to_parse = next_to_parse;
        }

        NumberSequence { elems: elems }
    }

    fn process_number(regex_match: (uint, uint),
                      to_parse: String) -> (StringElem, String) {
        let (_, end_index) = regex_match;
        let prefix_to_num: int = from_str(to_parse.slice_to(end_index))
                                    .unwrap();

        let next_token = StringElem::Number(prefix_to_num);
        let to_parse_suffix = to_parse.slice_from(end_index).to_string();

        (next_token, to_parse_suffix)
    }

    fn process_letters(regex_match: (uint, uint),
                       to_parse: String) -> (StringElem, String) {
        let (_, end_index) = regex_match;
        let prefix = to_parse.slice_to(end_index);

        let next_token = StringElem::Letters(prefix.to_string());
        let to_parse_suffix = to_parse.slice_from(end_index).to_string();

        (next_token, to_parse_suffix)
    }
}


#[test]
fn test_makes_numseq() {
    let str1 = "123";
    let seq1 = NumberSequence { elems: vec![StringElem::Number(123)] };
    assert_eq!(NumberSequence::from_str(str1), seq1);

    let str2 = "abc";
    let seq2 = NumberSequence {
        elems: vec![StringElem::Letters("abc".to_string())]
    };
    assert_eq!(NumberSequence::from_str(str2), seq2);

    let str3 = "abc123";
    let seq3 = NumberSequence {
        elems: vec![StringElem::Letters("abc".to_string()),
                    StringElem::Number(123)]
    };
    assert_eq!(NumberSequence::from_str(str3), seq3);
}
