#[deriving(Show, PartialEq, Eq)]
enum StringElem {
    Letters(String),
    Number(int)
}

/// A `HumanString` is a sort of string-like object that can be compared in a
/// human-friendly way.
#[deriving(Show, PartialEq, Eq)]
pub struct HumanString {
    elems: Vec<StringElem>
}

impl PartialOrd for HumanString {
    /// `HumanString`s are ordered based on their sub-components (a
    /// `HumanString` is represented as a sequence of numbers and strings). If
    /// two strings have analogous components, then they can be compared:
    ///
    /// ```
    /// use natural_sort::HumanString;
    ///
    /// assert!(HumanString::from_str("a1a") < HumanString::from_str("a1b"));
    /// assert!(HumanString::from_str("a11") > HumanString::from_str("a2"));
    /// ```
    ///
    /// However, `HumanString`s cannot always be compared. If the components of
    /// two strings do not match before a difference is found, then no
    /// comparison can be made:
    ///
    /// ```
    /// use natural_sort::HumanString;
    ///
    /// let a = HumanString::from_str("123");
    /// let b = HumanString::from_str("abc");
    ///
    /// assert_eq!(a.partial_cmp(&b), None);
    /// ```
    fn partial_cmp(&self, other: &HumanString) -> Option<Ordering> {
        let pairs = self.elems.iter().zip(other.elems.iter());
        let mut compares = pairs.map(|pair|
            match pair {
                (&StringElem::Number(a), &StringElem::Number(b)) => {
                    a.partial_cmp(&b)
                },

                (&StringElem::Letters(ref a), &StringElem::Letters(ref b)) => {
                    a.partial_cmp(b)
                },

                _ => { None }
            }
        );

        for comparison in compares {
            match comparison {
                None => { return None; }
                Some(Less) => { return Some(Less); }
                Some(Greater) => { return Some(Greater); }
                _ => { }
            }
        }

        self.elems.len().partial_cmp(&other.elems.len())
    }
}

impl HumanString {
    /// Constructs a `HumanString` from a `&str`.
    pub fn from_str(string: &str) -> HumanString {
        let numbers_re = regex!(r"^\p{N}+");
        let letters_re = regex!(r"^\P{N}+");

        let mut elems = Vec::new();
        let mut to_parse = String::from_str(string);

        while !to_parse.is_empty() {
            let numbers_match = numbers_re.find(to_parse.as_slice());

            let (next_token, next_to_parse) = if numbers_match.is_some() {
                HumanString::process_number(
                    numbers_match.unwrap(), to_parse)
            } else {
                let letters_match = letters_re.find(to_parse.as_slice());
                HumanString::process_letters(
                   letters_match.unwrap(), to_parse)
            };

            elems.push(next_token);
            to_parse = next_to_parse;
        }

        HumanString { elems: elems }
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

/// A utility function for sorting a list of strings using human sorting.
///
/// ```
/// use natural_sort::natural_sort;
///
/// let mut files = ["file1.txt", "file11.txt", "file2.txt"];
/// natural_sort(&mut files);
///
/// assert_eq!(files, ["file1.txt", "file2.txt", "file11.txt"]);
/// ```
pub fn natural_sort(strs: &mut [&str]) {
    fn sort_fn(a: &&str, b: &&str) -> Ordering {
        let seq_a = HumanString::from_str(*a);
        let seq_b = HumanString::from_str(*b);

        seq_a.partial_cmp(&seq_b).unwrap()
    }

    strs.sort_by(sort_fn);
}

#[test]
fn test_makes_numseq() {
    let str1 = "123";
    let hstr1 = HumanString { elems: vec![StringElem::Number(123)] };
    assert_eq!(HumanString::from_str(str1), hstr1);

    let str2 = "abc";
    let hstr2 = HumanString {
        elems: vec![StringElem::Letters("abc".to_string())]
    };
    assert_eq!(HumanString::from_str(str2), hstr2);

    let str3 = "abc123xyz456";
    let hstr3 = HumanString {
        elems: vec![StringElem::Letters("abc".to_string()),
                    StringElem::Number(123),
                    StringElem::Letters("xyz".to_string()),
                    StringElem::Number(456)]
    };
    assert_eq!(HumanString::from_str(str3), hstr3);
}

#[test]
fn test_compares_numseq() {
    fn compare_numseq(str1: &str, str2: &str) -> Option<Ordering> {
        HumanString::from_str(str1).partial_cmp(
            &HumanString::from_str(str2))
    }

    assert_eq!(compare_numseq("aaa", "aaa"), Some(Equal));
    assert_eq!(compare_numseq("aaa", "aab"), Some(Less));
    assert_eq!(compare_numseq("aab", "aaa"), Some(Greater));
    assert_eq!(compare_numseq("aaa", "aa"), Some(Greater));

    assert_eq!(compare_numseq("111", "111"), Some(Equal));
    assert_eq!(compare_numseq("111", "112"), Some(Less));
    assert_eq!(compare_numseq("112", "111"), Some(Greater));

    assert_eq!(compare_numseq("a1", "a1"), Some(Equal));
    assert_eq!(compare_numseq("a1", "a2"), Some(Less));
    assert_eq!(compare_numseq("a2", "a1"), Some(Greater));

    assert_eq!(compare_numseq("1a2", "1b1"), Some(Less));

    assert_eq!(compare_numseq("1", "a"), None);
}

#[test]
fn test_natural_sort() {
    let mut files = ["file1.txt", "file11.txt", "file2.txt"];
    natural_sort(&mut files);

    assert_eq!(files, ["file1.txt", "file2.txt", "file11.txt"]);
}
