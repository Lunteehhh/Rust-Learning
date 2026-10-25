use primzahlen::ist_primzahl::v4;

fn primzahl_loop(start: u32) {
    if start < 1 {
        panic!("Der Start-Wert darf nicht kleiner als 1 sein!");
    }

    let mut n = start;
    loop {
        if v4(n) { 
            println!("{n}");
        }
        n += 1;
    }

}

fn main() {
    let start = 810974;
    
    primzahl_loop(start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Der Start-Wert darf nicht kleiner als 1 sein!")]
    fn primzahl_loop_test() {
        primzahl_loop(0);
    }

}
