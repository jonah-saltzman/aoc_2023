macro_rules! value {
    ($c:expr) => {
        $c as u8 - '0' as u8
    };
}

macro_rules! select_match {
    ($dig: ident, $str: ident, $op:tt) => {
        match ($dig, $str) {
            (Some(dig), Some(str)) => {
                if dig.idx $op str.idx {
                    dig
                } else {
                    str
                }
            }
            (Some(dig), None) => dig,
            (None, Some(str)) => str,
            _ => unreachable!(),
        }
    };
}

type Digit = &'static str;

const DIGITS: [Digit; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Clone, Copy)]
struct StrMatch {
    idx: usize,
    val: usize,
}

fn digit_indices(line: &[u8]) -> (Option<StrMatch>, Option<StrMatch>) {
    let mut left: Option<StrMatch> = None;
    let mut right: Option<StrMatch> = None;
    for (i, &n) in line.iter().enumerate() {
        if n.is_ascii_digit() {
            left = Some(StrMatch {
                idx: i,
                val: value!(n) as usize,
            });
            break;
        }
    }

    for (i, &n) in line.iter().enumerate().rev() {
        if n.is_ascii_digit() {
            right = Some(StrMatch {
                idx: i,
                val: value!(n) as usize,
            });
            break;
        }
    }

    (left, right)
}

fn str_indices(line: &str) -> (Option<StrMatch>, Option<StrMatch>) {
    let mut left_match: Option<StrMatch> = None;
    let mut right_match: Option<StrMatch> = None;
    for (i, &dig) in DIGITS.iter().enumerate() {
        let l_idx = line.find(dig);
        let r_idx = line.rfind(dig);
        left_match = match (l_idx, left_match) {
            (Some(l), Some(mch)) if l < mch.idx => Some(StrMatch { idx: l, val: i }),
            (Some(l), None) => Some(StrMatch { idx: l, val: i }),
            _ => left_match,
        };
        right_match = match (r_idx, right_match) {
            (Some(r), Some(mch)) if r > mch.idx => Some(StrMatch { idx: r, val: i }),
            (Some(r), None) => Some(StrMatch { idx: r, val: i }),
            _ => right_match,
        }
    }

    (left_match, right_match)
}

pub fn parse_line(line: &str) -> usize {
    let (left_dig, right_dig) = digit_indices(line.as_bytes());
    let (left_str, right_str) = str_indices(line);

    let left = select_match!(left_dig, left_str, <);
    let right = select_match!(right_dig, right_str, >);

    left.val * 10 + right.val
}
