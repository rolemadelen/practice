use std::io::{self ,Write};

fn reads() -> String {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s).unwrap();
    s.trim().to_ascii_lowercase()
}

fn main() -> io::Result<()> {
    let s = reads();
    let mut freq = [0usize; 26];

    for ch in s.chars().filter(|c| c.is_ascii_lowercase()) {
        let idx = (ch as u8 - b'a') as usize;
        freq[idx] += 1;
    }

    let max = freq.iter().copied().max().unwrap_or(0);

    let mut max_chars = freq
        .iter()
        .enumerate()
        .filter(|&(_, &count)| count == max && max > 0)
        .map(|(i, _)| (b'A' + i as u8) as char);

    let result = match (max_chars.next(), max_chars.next()) {
        (Some(c), None) => c.to_string(),
        _ => "?".to_string()
    };

    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(out, "{result}")?;

    Ok(())
}