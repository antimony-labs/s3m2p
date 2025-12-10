//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | ML/antimony-core/src/lib.rs
//! PURPOSE: Library crate root module with public API exports
//! MODIFIED: 2025-11-26
//! LAYER: LEARN → ML
//! ═══════════════════════════════════════════════════════════════════════════════
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
