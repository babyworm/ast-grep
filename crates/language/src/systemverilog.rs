#![cfg(test)]
use super::*;

fn test_match(query: &str, source: &str) {
  use crate::test::test_match_lang;
  test_match_lang(query, source, SystemVerilog);
}

fn test_non_match(query: &str, source: &str) {
  use crate::test::test_non_match_lang;
  test_non_match_lang(query, source, SystemVerilog);
}

#[test]
fn test_sv_simple() {
  test_match("module $A; endmodule", "module test; endmodule");
  // Use 'input $B' to match structure of 'input clk'
  test_match("module $A (input $B); endmodule", "module test (input clk); endmodule");
  test_non_match("module $A; endmodule", "program test; endprogram");
}

#[test]
fn test_sv_replace() {
  use crate::test::test_replace_lang;
  fn test_replace(src: &str, pattern: &str, replacer: &str) -> String {
    test_replace_lang(src, pattern, replacer, SystemVerilog)
  }

  // Use new_$A to verify replacement (prefix)
  let ret = test_replace(
    "module test; endmodule",
    "module $A; endmodule",
    "module new_$A; endmodule",
  );
  assert_eq!(ret, "module new_test; endmodule");
}
