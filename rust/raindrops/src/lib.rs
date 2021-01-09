pub fn raindrops(n: u32) -> String {
    let factors = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let output = factors
        .iter()
        .map(|factor| if n % factor.0 == 0 { factor.1 } else { "" })
        .collect::<String>();

    match output.is_empty() {
        true => n.to_string(),
        false => output,
    }
}
