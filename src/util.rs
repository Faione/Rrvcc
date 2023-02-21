use std::{iter::Peekable, str::Chars};

/// 遍历字符串数组，匹配连续的数字，将其转化为10进制数进行输出
/// 持续移动迭代器, 直到 `chars.peek()` 非数字
pub fn strol(chars: &mut Peekable<Chars<'_>>) -> i64 {
    let mut result: i64 = 0;
    loop {
        if let Some(c) = chars.peek() {
            if let Some(i) = c.to_digit(10) {
                result = result * 10 + i64::from(i);
                chars.next();
                continue;
            }
        }

        break;
    }

    result
}

#[cfg(test)]
mod test {
    use super::strol;

    #[test]
    fn test_strol() {
        let str = String::from("123abc456");

        let mut chars = str.chars().peekable();

        let first = strol(&mut chars);
        assert_eq!(first, 123);
    }
}
