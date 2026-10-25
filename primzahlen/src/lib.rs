pub mod ist_primzahl {

    pub fn v1(nummer: u32) -> bool {
        for i in 2..nummer {
            let komma_wert = nummer as f64 / i as f64;
            let ganz_wert = komma_wert as u32;

            if komma_wert == ganz_wert as f64 { return false; } 
        }
        true
    }

    pub fn v2(nummer: u32) -> bool {
        if nummer.is_multiple_of(2) {
            if nummer == 2 {
                return true;
            }
            return false;
        }
        
        for i in 3..nummer {
            let komma_wert = nummer as f64 / i as f64;
            let ganz_wert = komma_wert as u32;

            if komma_wert == ganz_wert as f64 { return false; } 
        }
        true
    }

    pub fn v3(nummer: u32) -> bool {
        for i in 2..nummer {
            if nummer.is_multiple_of(i) {
                return false;
            }
        }
        true
    }

    pub fn v4(nummer: u32) -> bool {
        for i in 2..(nummer.isqrt()+1) {
            if nummer.is_multiple_of(i) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::ist_primzahl::*;
    
    mod test_v1 {
        use super::*;
        
        #[test]
        fn ist_primzahl_1_test() {
            let result = v1(1); 

            assert!(result, "1 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_2_test() {
            let result = v1(2); 

            assert!(result, "2 ist eine Primzahl.")
        }
        
        #[test]
        fn ist_primzahl_3_test() {
            let result = v1(7); 

            assert!(result, "7 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_4_test() {
            let result = v1(9);

            assert!(!result, "9 ist keine Primzahl.")
        }

        #[test]
        #[ignore]
        fn ist_primzahl_5_test() {
            let result = v1(4_294_967_291);

            assert!(result, "4_294_967_291 ist eine Primzahl.")

        }
    }

    mod test_v2 {
        use super::*;

        #[test]
        fn ist_primzahl_1_test() {
            let result = v2(1); 

            assert!(result, "1 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_2_test() {
            let result = v2(2); 

            assert!(result, "2 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_3_test() {
            let result = v1(7); 

            assert!(result, "7 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_4_test() {
            let result = v2(9);

            assert!(!result, "9 ist keine Primzahl.")
        }

        #[test]
        #[ignore]
        fn ist_primzahl_5_test() {
            let result = v2(4_294_967_291);

            assert!(result, "4_294_967_291 ist eine Primzahl.")

        }

    }
    mod test_v3 {
        use super::*;

        #[test]
        fn ist_primzahl_1_test() {
            let result = v3(1); 

            assert!(result, "1 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_2_test() {
            let result = v3(2); 

            assert!(result, "2 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_3_test() {
            let result = v1(7); 

            assert!(result, "7 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_4_test() {
            let result = v3(9);

            assert!(!result, "9 ist keine Primzahl.")
        }

        #[test]
        #[ignore]
        fn ist_primzahl_5_test() {
            let result = v3(4_294_967_291);

            assert!(result, "4_294_967_291 ist eine Primzahl.")

        }

    }
    mod test_v4 {
        use super::*;

        #[test]
        fn ist_primzahl_1_test() {
            let result = v4(1); 

            assert!(result, "1 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_2_test() {
            let result = v4(2); 

            assert!(result, "2 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_3_test() {
            let result = v4(7); 

            assert!(result, "7 ist eine Primzahl.")
        }

        #[test]
        fn ist_primzahl_4_test() {
            let result = v4(9);

            assert!(!result, "9 ist keine Primzahl.")
        }

        #[test]
        #[ignore]
        fn ist_primzahl_5_test() {
            let result = v4(4_294_967_291);

            assert!(result, "4_294_967_291 ist eine Primzahl.")

        }
    }
}
