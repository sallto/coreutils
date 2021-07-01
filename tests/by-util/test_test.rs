//
// This file is part of the uutils coreutils package.
//
// (c) mahkoh (ju.orth [at] gmail [dot] com)
// (c) Daniel Rocco <drocco@gmail.com>
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
//

// spell-checker:ignore (words) egid euid pseudofloat

use crate::common::util::*;

#[test]
fn test_empty_test_equivalent_to_false() {
    new_ucmd!().run().status_code(1);
}

#[test]
fn test_empty_string_is_false() {
    new_ucmd!().arg("").run().status_code(1);
}

#[test]
fn test_solo_not() {
    new_ucmd!().arg("!").succeeds();
}

#[test]
fn test_solo_and_or_or_is_a_literal() {
    // /bin/test '' -a '' => 1; so test(1) must interpret `-a` by itself as
    // a literal string
    new_ucmd!().arg("-a").succeeds();
    new_ucmd!().arg("-o").succeeds();
}

#[test]
fn test_double_not_is_false() {
    new_ucmd!().args(&["!", "!"]).run().status_code(1);
}

#[test]
fn test_and_not_is_false() {
    new_ucmd!().args(&["-a", "!"]).run().status_code(1);
}

#[test]
fn test_not_and_is_false() {
    // `-a` is a literal here & has nonzero length
    new_ucmd!().args(&["!", "-a"]).run().status_code(1);
}

#[test]
fn test_not_and_not_succeeds() {
    new_ucmd!().args(&["!", "-a", "!"]).succeeds();
}

#[test]
fn test_simple_or() {
    new_ucmd!().args(&["foo", "-o", ""]).succeeds();
}

#[test]
fn test_negated_or() {
    new_ucmd!()
        .args(&["!", "foo", "-o", "bar"])
        .run()
        .status_code(1);
    new_ucmd!().args(&["foo", "-o", "!", "bar"]).succeeds();
    new_ucmd!()
        .args(&["!", "foo", "-o", "!", "bar"])
        .run()
        .status_code(1);
}

#[test]
fn test_string_length_of_nothing() {
    // odd but matches GNU, which must interpret -n as a literal here
    new_ucmd!().arg("-n").succeeds();
}

#[test]
fn test_string_length_of_empty() {
    new_ucmd!().args(&["-n", ""]).run().status_code(1);

    // STRING equivalent to -n STRING
    new_ucmd!().arg("").run().status_code(1);
}

#[test]
fn test_nothing_is_empty() {
    // -z is a literal here and has nonzero length
    new_ucmd!().arg("-z").succeeds();
}

#[test]
fn test_zero_len_of_empty() {
    new_ucmd!().args(&["-z", ""]).succeeds();
}

#[test]
fn test_solo_parenthesis_is_literal() {
    let scenario = TestScenario::new(util_name!());
    let tests = [["("], [")"]];

    for test in &tests {
        scenario.ucmd().args(&test[..]).succeeds();
    }
}

#[test]
fn test_solo_empty_parenthetical_is_error() {
    new_ucmd!().args(&["(", ")"]).run().status_code(2);
}

#[test]
fn test_zero_len_equals_zero_len() {
    new_ucmd!().args(&["", "=", ""]).succeeds();
}

#[test]
fn test_zero_len_not_equals_zero_len_is_false() {
    new_ucmd!().args(&["", "!=", ""]).run().status_code(1);
}

#[test]
fn test_double_equal_is_string_comparison_op() {
    // undocumented but part of the GNU test suite
    new_ucmd!().args(&["t", "==", "t"]).succeeds();
    new_ucmd!().args(&["t", "==", "f"]).run().status_code(1);
}

#[test]
fn test_string_comparison() {
    let scenario = TestScenario::new(util_name!());
    let tests = [
        ["foo", "!=", "bar"],
        ["contained\nnewline", "=", "contained\nnewline"],
        ["(", "=", "("],
        ["(", "!=", ")"],
        ["!", "=", "!"],
        ["=", "=", "="],
    ];

    for test in &tests {
        scenario.ucmd().args(&test[..]).succeeds();
    }

    // run the inverse of all these tests
    for test in &tests {
        scenario
            .ucmd()
            .arg("!")
            .args(&test[..])
            .run()
            .status_code(1);
    }
}

#[test]
#[ignore = "fixme: error reporting"]
fn test_dangling_string_comparison_is_error() {
    new_ucmd!()
        .args(&["missing_something", "="])
        .run()
        .status_code(2)
        .stderr_is("test: missing argument after '='");
}

#[test]
fn test_string_operator_is_literal_after_bang() {
    let scenario = TestScenario::new(util_name!());
    let tests = [
        ["!", "="],
        ["!", "!="],
        ["!", "-eq"],
        ["!", "-ne"],
        ["!", "-lt"],
        ["!", "-le"],
        ["!", "-gt"],
        ["!", "-ge"],
        ["!", "-ef"],
        ["!", "-nt"],
        ["!", "-ot"],
    ];

    for test in &tests {
        scenario.ucmd().args(&test[..]).run().status_code(1);
    }
}

#[test]
fn test_a_bunch_of_not() {
    new_ucmd!()
        .args(&["!", "", "!=", "", "-a", "!", "", "!=", ""])
        .succeeds();
}

#[test]
fn test_pseudofloat_equal() {
    new_ucmd!().args(&["123.45", "=", "123.45"]).succeeds();
}

#[test]
fn test_pseudofloat_not_equal() {
    new_ucmd!().args(&["123.45", "!=", "123.450"]).succeeds();
}

#[test]
fn test_negative_arg_is_a_string() {
    new_ucmd!().arg("-12345").succeeds();
    new_ucmd!().arg("--qwert").succeeds(); // spell-checker:disable-line
}

#[test]
fn test_some_int_compares() {
    let scenario = TestScenario::new(util_name!());

    let tests = [
        ["0", "-eq", "0"],
        ["0", "-ne", "1"],
        ["421", "-lt", "3720"],
        ["0", "-le", "0"],
        ["11", "-gt", "10"],
        ["1024", "-ge", "512"],
        ["9223372036854775806", "-le", "9223372036854775807"],
    ];

    for test in &tests {
        scenario.ucmd().args(&test[..]).succeeds();
    }
}

#[test]
#[ignore = "fixme: evaluation error (code 1); GNU returns 0"]
fn test_values_greater_than_i64_allowed() {
    new_ucmd!()
        .args(&["9223372036854775808", "-gt", "0"])
        .succeeds();
}

#[test]
fn test_negative_int_compare() {
    let scenario = TestScenario::new(util_name!());

    let tests = [
        ["-1", "-eq", "-1"],
        ["-1", "-ne", "-2"],
        ["-3720", "-lt", "-421"],
        ["-10", "-le", "-10"],
        ["-21", "-gt", "-22"],
        ["-128", "-ge", "-256"],
        ["-9223372036854775808", "-le", "-9223372036854775807"],
    ];

    for test in &tests {
        scenario.ucmd().args(&test[..]).succeeds();
    }
}

#[test]
fn test_float_inequality_is_error() {
    new_ucmd!()
        .args(&["123.45", "-ge", "6"])
        .run()
        .status_code(2)
        .stderr_is("test: invalid integer '123.45'");
}

#[test]
#[cfg(not(windows))]
fn test_invalid_utf8_integer_compare() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let source = [0x66, 0x6f, 0x80, 0x6f];
    let arg = OsStr::from_bytes(&source[..]);

    let mut cmd = new_ucmd!();
    cmd.arg("123").arg("-ne");
    cmd.raw.arg(arg);

    cmd.run()
        .status_code(2)
        .stderr_is("test: invalid integer 'fo�o'");

    let mut cmd = new_ucmd!();
    cmd.raw.arg(arg);
    cmd.arg("-eq").arg("456");

    cmd.run()
        .status_code(2)
        .stderr_is("test: invalid integer 'fo�o'");
}

#[test]
#[ignore = "fixme: parse/evaluation error (code 2); GNU returns 1"]
fn test_file_is_itself() {
    new_ucmd!()
        .args(&["regular_file", "-ef", "regular_file"])
        .succeeds();
}

#[test]
#[ignore = "fixme: parse/evaluation error (code 2); GNU returns 1"]
fn test_file_is_newer_than_and_older_than_itself() {
    // odd but matches GNU
    new_ucmd!()
        .args(&["regular_file", "-nt", "regular_file"])
        .run()
        .status_code(1);
    new_ucmd!()
        .args(&["regular_file", "-ot", "regular_file"])
        .run()
        .status_code(1);
}

#[test]
#[ignore = "todo: implement these"]
fn test_newer_file() {
    let scenario = TestScenario::new(util_name!());

    scenario.cmd("touch").arg("newer_file").succeeds();
    scenario
        .cmd("touch")
        .args(&["-m", "-d", "last Thursday", "regular_file"])
        .succeeds();

    scenario
        .ucmd()
        .args(&["newer_file", "-nt", "regular_file"])
        .succeeds();
    scenario
        .ucmd()
        .args(&["regular_file", "-ot", "newer_file"])
        .succeeds();
}

#[test]
fn test_file_exists() {
    new_ucmd!().args(&["-e", "regular_file"]).succeeds();
}

#[test]
fn test_nonexistent_file_does_not_exist() {
    new_ucmd!()
        .args(&["-e", "nonexistent_file"])
        .run()
        .status_code(1);
}

#[test]
fn test_nonexistent_file_is_not_regular() {
    new_ucmd!()
        .args(&["-f", "nonexistent_file"])
        .run()
        .status_code(1);
}

#[test]
fn test_file_exists_and_is_regular() {
    new_ucmd!().args(&["-f", "regular_file"]).succeeds();
}

#[test]
#[cfg(not(windows))] // FIXME: implement on Windows
fn test_file_is_readable() {
    new_ucmd!().args(&["-r", "regular_file"]).succeeds();
}

#[test]
#[cfg(not(windows))] // FIXME: implement on Windows
fn test_file_is_not_readable() {
    let scenario = TestScenario::new(util_name!());
    let mut ucmd = scenario.ucmd();
    let mut chmod = scenario.cmd("chmod");

    scenario.fixtures.touch("crypto_file");
    chmod.args(&["u-r", "crypto_file"]).succeeds();

    ucmd.args(&["!", "-r", "crypto_file"]).succeeds();
}

#[test]
#[cfg(not(windows))] // FIXME: implement on Windows
fn test_file_is_writable() {
    new_ucmd!().args(&["-w", "regular_file"]).succeeds();
}

#[test]
#[cfg(not(windows))] // FIXME: implement on Windows
fn test_file_is_not_writable() {
    let scenario = TestScenario::new(util_name!());
    let mut ucmd = scenario.ucmd();
    let mut chmod = scenario.cmd("chmod");

    scenario.fixtures.touch("immutable_file");
    chmod.args(&["u-w", "immutable_file"]).succeeds();

    ucmd.args(&["!", "-w", "immutable_file"]).succeeds();
}

#[test]
fn test_file_is_not_executable() {
    new_ucmd!().args(&["!", "-x", "regular_file"]).succeeds();
}

#[test]
#[cfg(not(windows))] // FIXME: implement on Windows
fn test_file_is_executable() {
    let scenario = TestScenario::new(util_name!());
    let mut chmod = scenario.cmd("chmod");

    chmod.args(&["u+x", "regular_file"]).succeeds();

    scenario.ucmd().args(&["-x", "regular_file"]).succeeds();
}

#[test]
fn test_is_not_empty() {
    new_ucmd!().args(&["-s", "non_empty_file"]).succeeds();
}

#[test]
fn test_nonexistent_file_size_test_is_false() {
    new_ucmd!()
        .args(&["-s", "nonexistent_file"])
        .run()
        .status_code(1);
}

#[test]
fn test_not_is_not_empty() {
    new_ucmd!().args(&["!", "-s", "regular_file"]).succeeds();
}

#[test]
#[cfg(not(windows))]
fn test_symlink_is_symlink() {
    let scenario = TestScenario::new(util_name!());
    let at = &scenario.fixtures;

    at.symlink_file("regular_file", "symlink");

    // FIXME: implement on Windows
    scenario.ucmd().args(&["-h", "symlink"]).succeeds();
    scenario.ucmd().args(&["-L", "symlink"]).succeeds();
}

#[test]
fn test_file_is_not_symlink() {
    let scenario = TestScenario::new(util_name!());

    scenario
        .ucmd()
        .args(&["!", "-h", "regular_file"])
        .succeeds();
    scenario
        .ucmd()
        .args(&["!", "-L", "regular_file"])
        .succeeds();
}

#[test]
fn test_nonexistent_file_is_not_symlink() {
    let scenario = TestScenario::new(util_name!());

    scenario
        .ucmd()
        .args(&["!", "-h", "nonexistent_file"])
        .succeeds();
    scenario
        .ucmd()
        .args(&["!", "-L", "nonexistent_file"])
        .succeeds();
}

#[test]
#[cfg(not(windows))] // Windows has no concept of sticky bit
fn test_file_is_sticky() {
    let scenario = TestScenario::new(util_name!());
    let mut ucmd = scenario.ucmd();
    let mut chmod = scenario.cmd("chmod");

    scenario.fixtures.touch("sticky_file");
    chmod.args(&["+t", "sticky_file"]).succeeds();

    ucmd.args(&["-k", "sticky_file"]).succeeds();
}

#[test]
fn test_file_is_not_sticky() {
    new_ucmd!()
        .args(&["-k", "regular_file"])
        .run()
        .status_code(1);
}

#[test]
#[cfg(not(windows))]
fn test_file_owned_by_euid() {
    new_ucmd!().args(&["-O", "regular_file"]).succeeds();
}

#[test]
#[cfg(not(windows))]
fn test_nonexistent_file_not_owned_by_euid() {
    new_ucmd!()
        .args(&["-O", "nonexistent_file"])
        .run()
        .status_code(1);
}

#[test]
#[cfg(all(not(windows), not(target_os = "freebsd")))]
fn test_file_not_owned_by_euid() {
    new_ucmd!()
        .args(&["-f", "/bin/sh", "-a", "!", "-O", "/bin/sh"])
        .succeeds();
}

#[test]
#[cfg(not(windows))]
fn test_file_owned_by_egid() {
    new_ucmd!().args(&["-G", "regular_file"]).succeeds();
}

#[test]
#[cfg(not(windows))]
fn test_nonexistent_file_not_owned_by_egid() {
    new_ucmd!()
        .args(&["-G", "nonexistent_file"])
        .run()
        .status_code(1);
}

#[test]
#[cfg(all(not(windows), not(target_os = "freebsd")))]
fn test_file_not_owned_by_egid() {
    new_ucmd!()
        .args(&["-f", "/bin/sh", "-a", "!", "-G", "/bin/sh"])
        .succeeds();
}

#[test]
fn test_op_precedence_and_or_1() {
    new_ucmd!().args(&[" ", "-o", "", "-a", ""]).succeeds();
}

#[test]
fn test_op_precedence_and_or_1_overridden_by_parentheses() {
    new_ucmd!()
        .args(&["(", " ", "-o", "", ")", "-a", ""])
        .run()
        .status_code(1);
}

#[test]
fn test_op_precedence_and_or_2() {
    new_ucmd!()
        .args(&["", "-a", "", "-o", " ", "-a", " "])
        .succeeds();
}

#[test]
fn test_op_precedence_and_or_2_overridden_by_parentheses() {
    new_ucmd!()
        .args(&["", "-a", "(", "", "-o", " ", ")", "-a", " "])
        .run()
        .status_code(1);
}

#[test]
fn test_negated_boolean_precedence() {
    let scenario = TestScenario::new(util_name!());

    let tests = [
        vec!["!", "(", "foo", ")", "-o", "bar"],
        vec!["!", "", "-o", "", "-a", ""],
        vec!["!", "(", "", "-a", "", ")", "-o", ""],
    ];

    for test in &tests {
        scenario.ucmd().args(&test[..]).succeeds();
    }

    let negative_tests = [
        vec!["!", "-n", "", "-a", ""],
        vec!["", "-a", "", "-o", ""],
        vec!["!", "", "-a", "", "-o", ""],
        vec!["!", "(", "", "-a", "", ")", "-a", ""],
    ];

    for test in &negative_tests {
        scenario.ucmd().args(&test[..]).run().status_code(1);
    }
}

#[test]
fn test_bang_bool_op_precedence() {
    // For a Boolean combination of two literals, bang inverts the entire expression
    new_ucmd!().args(&["!", "", "-a", ""]).succeeds();
    new_ucmd!().args(&["!", "", "-o", ""]).succeeds();

    new_ucmd!()
        .args(&["!", "a value", "-o", "another value"])
        .run()
        .status_code(1);

    // Introducing a UOP — even one that is equivalent to a bare string — causes
    // bang to invert only the first term
    new_ucmd!()
        .args(&["!", "-n", "", "-a", ""])
        .run()
        .status_code(1);
    new_ucmd!()
        .args(&["!", "", "-a", "-n", ""])
        .run()
        .status_code(1);

    // for compound Boolean expressions, bang inverts the _next_ expression
    // only, not the entire compound expression
    new_ucmd!()
        .args(&["!", "", "-a", "", "-a", ""])
        .run()
        .status_code(1);

    // parentheses can override this
    new_ucmd!()
        .args(&["!", "(", "", "-a", "", "-a", "", ")"])
        .succeeds();
}

#[test]
fn test_inverted_parenthetical_bool_op_precedence() {
    // For a Boolean combination of two literals, bang inverts the entire expression
    new_ucmd!()
        .args(&["!", "a value", "-o", "another value"])
        .run()
        .status_code(1);

    // only the parenthetical is inverted, not the entire expression
    new_ucmd!()
        .args(&["!", "(", "a value", ")", "-o", "another value"])
        .succeeds();
}

#[test]
#[ignore = "fixme: error reporting"]
fn test_dangling_parenthesis() {
    new_ucmd!()
        .args(&["(", "(", "a", "!=", "b", ")", "-o", "-n", "c"])
        .run()
        .status_code(2);
    new_ucmd!()
        .args(&["(", "(", "a", "!=", "b", ")", "-o", "-n", "c", ")"])
        .succeeds();
}

#[test]
fn test_complicated_parenthesized_expression() {
    new_ucmd!()
        .args(&[
            "(", "(", "!", "(", "a", "=", "b", ")", "-o", "c", "=", "d", ")", "-a", "(", "q", "!=",
            "r", ")", ")",
        ])
        .succeeds();
}

#[test]
fn test_erroneous_parenthesized_expression() {
    new_ucmd!()
        .args(&["a", "!=", "(", "b", "-a", "b", ")", "!=", "c"])
        .run()
        .status_code(2)
        .stderr_is("test: extra argument 'b'");
}

#[test]
fn test_or_as_filename() {
    new_ucmd!()
        .args(&["x", "-a", "-z", "-o"])
        .run()
        .status_code(1);
}

#[test]
#[ignore = "GNU considers this an error"]
fn test_string_length_and_nothing() {
    new_ucmd!().args(&["-n", "a", "-a"]).run().status_code(2);
}

#[test]
fn test_bracket_syntax_success() {
    let scenario = TestScenario::new("[");
    let mut ucmd = scenario.ucmd();

    ucmd.args(&["1", "-eq", "1", "]"]).succeeds();
}

#[test]
fn test_bracket_syntax_failure() {
    let scenario = TestScenario::new("[");
    let mut ucmd = scenario.ucmd();

    ucmd.args(&["1", "-eq", "2", "]"]).run().status_code(1);
}

#[test]
fn test_bracket_syntax_missing_right_bracket() {
    let scenario = TestScenario::new("[");
    let mut ucmd = scenario.ucmd();

    // Missing closing bracket takes precedence over other possible errors.
    ucmd.args(&["1", "-eq"])
        .run()
        .status_code(2)
        .stderr_is("[: missing ']'");
}
