use core::fmt;
use std::fmt::Write;
struct EnglishNumeral {
    num: u32,
    capitalized: bool,
}
impl fmt::Display for EnglishNumeral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ONES_NUMERALS: [&str; 10] = [
            "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        const CAPITALIZED_ONES_NUMERALS: [&str; 10] = [
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        ];

        const TENS_NUMERALS: [&str; 10] = [
            "", "ten", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eighty",
            "ninety",
        ];
        const CAPITALIZED_TENS_NUMERALS: [&str; 10] = [
            "", "Ten", "Twenty", "Thirty", "Fourty", "Fifty", "Sixty", "Seventy", "Eighty",
            "Ninety",
        ];
        const TEENS_NUMERALS: [&str; 10] = [
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
        ];
        const CAPITALIZED_TEENS_NUMERALS: [&str; 10] = [
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];
        const SHORT_SCALE: [&str; 4] = ["", "thousand", "million", "billion"];
        if self.num == 0 {
            return f.write_str(if self.capitalized { "Zero" } else { "zero" });
        }
        let mut chunks_str = Vec::new();
        let digits = self.num.to_string();
        let mut should_capitalize = self.capitalized;
        for (scale, chunk) in digits.as_bytes().rchunks(3).enumerate().rev() {
            let mut buf = String::new();
            let chunk: Vec<&u8> = chunk.iter().rev().collect();
            let hundreds = *chunk.get(2).unwrap_or(&&b'0') - b'0';
            let tens = *chunk.get(1).unwrap_or(&&b'0') - b'0';
            let ones = *chunk.get(0).unwrap_or(&&b'0') - b'0';
            if hundreds != 0 {
                write!(
                    &mut buf,
                    "{} hundred and ",
                    if should_capitalize {
                        should_capitalize = false;
                        CAPITALIZED_ONES_NUMERALS[usize::from(hundreds)]
                    } else {
                        ONES_NUMERALS[usize::from(hundreds)]
                    }
                )?;
            }
            if tens == 1 {
                write!(
                    &mut buf,
                    "{}",
                    if should_capitalize {
                        should_capitalize = false;
                        CAPITALIZED_TEENS_NUMERALS[usize::from(ones)]
                    } else {
                        TEENS_NUMERALS[usize::from(ones)]
                    }
                )?;
            } else {
                if tens != 0 {
                    write!(
                        &mut buf,
                        "{}-",
                        if should_capitalize {
                            should_capitalize = false;
                            CAPITALIZED_TENS_NUMERALS[usize::from(tens)]
                        } else {
                            TENS_NUMERALS[usize::from(tens)]
                        }
                    )?;
                }
                write!(
                    &mut buf,
                    "{}",
                    if should_capitalize {
                        should_capitalize = false;
                        CAPITALIZED_ONES_NUMERALS[usize::from(ones)]
                    } else {
                        ONES_NUMERALS[usize::from(ones)]
                    }
                )?;
            }
            if scale != 0 {
                write!(&mut buf, " {}", SHORT_SCALE[scale])?;
            }
            chunks_str.push(buf);
        }
        let mut chunks_iter = chunks_str.iter();
        if let Some(chunk_str) = chunks_iter.next() {
            f.write_str(chunk_str)?;
        }
        for chunk_str in chunks_iter {
            write!(f, " {chunk_str}")?;
        }
        Ok(())
    }
}

#[test]
fn should_format_usize_max_correctly() {
    assert_eq!(
        format!(
            "{}",
            EnglishNumeral {
                num: u32::MAX,
                capitalized: false
            }
        ),
        "four billion two hundred and ninety-four million nine hundred and sixty-seven thousand two hundred and ninety-five"
    );
    assert_eq!(
        format!(
            "{}",
            EnglishNumeral {
                num: u32::MAX,
                capitalized: true
            }
        ),
        "Four billion two hundred and ninety-four million nine hundred and sixty-seven thousand two hundred and ninety-five"
    )
}
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    fn bottles(n: u32) -> String {
        (if n == 1 { "bottle" } else { "bottles" }).to_string()
    }
    if take_down > start_bottles {
        panic!(
            "Expected that the starting number of bottles {start_bottles} is less or equal the number of times we take down a bottle {take_down}"
        );
    } else {
        ((start_bottles - take_down + 1)..=start_bottles)
            .rev()
            .map(|num| {
                if num > 1 {
                    format!(
                        "{} green {num_bottles} hanging on the wall,\n\
                         {} green bottles hanging on the wall,\n\
                         And if one green bottle should accidentally fall,\n\
                         There'll be {} green {pred_num_bottles} hanging on the wall.\n\n",
                        EnglishNumeral {
                            num,
                            capitalized: true
                        },
                        EnglishNumeral {
                            num,
                            capitalized: true
                        },
                        EnglishNumeral {
                            num: num - 1,
                            capitalized: false
                        },
                        num_bottles = bottles(num),
                        pred_num_bottles = bottles(num - 1)
                    )
                } else if num == 1 {
                    "One green bottle hanging on the wall,\n\
                     One green bottle hanging on the wall,\n\
                     And if one green bottle should accidentally fall,\n\
                     There'll be no green bottles hanging on the wall.\n\n".to_string()
                } else {
                    "".to_string()
                }
            }).collect()
    }
}
