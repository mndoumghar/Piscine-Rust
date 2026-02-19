pub fn number_logic(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    let mut temp = num;
    let mut res = 0;

    while temp > 0 {
        let digit = temp % 10;
        res += digit.pow(len);
        temp /= 10;
    }

    res == num
}
