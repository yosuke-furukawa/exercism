pub fn build_proverb(list: &[&str]) -> String {
    let length = list.len() as i32;
    let mut index = 0;
    let mut result: Vec<String> = Vec::new();
    while index + 1 < length {
        let s = format!("For want of a {} the {} was lost.", list[index as usize], list[(index + 1) as usize]);
        result.push(s);
        index = index + 1;
    }
    if length > 0 {
        let s = format!("And all for the want of a {}.", list[0 as usize]);
        result.push(s);
    }

    result.join("\n")
}
