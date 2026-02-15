pub fn check_ms(word: &str) -> Result<&str, &str> {
    if word == "" {
        return Err("ERROR: illegal");
    }

    for wo in word.split_whitespace() {
        if  wo =="hello"{
            return Ok(wo);
        }

        if wo.contains("stupid") {
            return Err("ERROR: illegal");
        }
    }

    Ok(word)
}
