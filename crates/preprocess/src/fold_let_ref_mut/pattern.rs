
pub const PATTERNS: &[Pattern] = &[
    VALUE_ASSIGN_AS_ASSIGNER,
    VALUE_ASSIGN_IN_CONDITION,
    ASSIGN_TWO_LINES, 
    PTR_INCR_FOUR_LINES
];

pub struct Pattern {
    pub pattern: &'static str,
    pub replacer: for<'t> fn(&regex::Captures<'t>) -> String,
}

/// ```c
/// x = y = value;
/// ```
const VALUE_ASSIGN_AS_ASSIGNER: Pattern = Pattern {
    pattern: concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs1>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*(?P<rhs>[^;]+);[\s|\n]*",
        r"let ref mut fresh(?P<version3>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs2>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version4>[0-9]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version5>[0-9]+)[\s|\n]*;",
    ),
    replacer: value_assign_as_assigner,
};

fn value_assign_as_assigner(caps: &regex::Captures<'_>) -> String {
    let original = &caps[0];
    let version1 = &caps["version1"];
    let version2 = &caps["version2"];
    let version3 = &caps["version3"];
    let version4 = &caps["version4"];
    let version5 = &caps["version5"];

    if !(version1 == version2 && version1 != version3 && version3 == version4 && version1 == version5) {
        return original.to_owned();
    }


    // let expr1 = regex::Regex::new(&(r"\*fresh".to_owned() + version1)).unwrap();
    // let expr2 = regex::Regex::new(&(r"\*fresh".to_owned() + version3)).unwrap();

    let lhs1 = &caps["lhs1"];
    let lhs2 = &caps["lhs2"];
    let rhs = &caps["rhs"];


    lhs1.to_string() + " = " + rhs + "; " + lhs2 + " = " + lhs1 + ";"
}

/// ```c
/// if ((x = value))
/// ```
const VALUE_ASSIGN_IN_CONDITION: Pattern = Pattern {
    pattern: concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs>[^;]+);",
        r"[\s|\n]*\*fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*(?P<rhs>[^;]+);",
        r"(?P<line3>[\s|\n]*if[\s|\n]*\(\*fresh(?P<version3>[0-9]+)\).is_null\(\))"
    ),
    replacer: value_assign_in_condition,
};

fn value_assign_in_condition(caps: &regex::Captures<'_>) -> String {
    let original = &caps[0];
    let version1 = &caps["version1"];
    let version2 = &caps["version2"];
    let version3 = &caps["version3"];

    if version1 != version2 || version1 != version3 {
        return original.to_owned();
    }

    let expr_fresh = r"\*fresh".to_owned() + version1;
    let expr_fresh = regex::Regex::new(&expr_fresh).unwrap();

    let lhs = &caps["lhs"];
    let rhs = &caps["rhs"];
    let line3 = &caps["line3"];

    let rhs = expr_fresh.replace_all(rhs, lhs);
    let line3 = expr_fresh.replace_all(line3, lhs);

    lhs.to_owned() + " = " + &rhs + ";" + &line3
}

const ASSIGN_TWO_LINES: Pattern = Pattern {
    pattern: concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs>[^;]+);",
        r"[\s|\n]*\*fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*(?P<rhs>[^;]+);"
    ),
    replacer: assign_two_lines,
};

fn assign_two_lines(caps: &regex::Captures<'_>) -> String {
    let original = &caps[0];
    let version1 = &caps["version1"];
    let version2 = &caps["version2"];

    if version1 != version2 {
        return original.to_owned();
    }

    let expr_fresh = r"\*fresh".to_owned() + version1;

    let lhs = &caps["lhs"];
    let rhs = &caps["rhs"];

    let rhs = regex::Regex::new(&expr_fresh)
        .unwrap()
        .replace_all(rhs, lhs);

    lhs.to_owned() + " = " + &rhs + ";"
}

const PTR_INCR_FOUR_LINES: Pattern = Pattern {
    pattern: concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs>[^;]+);[\s|\n]*",
        r"let fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version3>[0-9]+)[^;]*;[\s|\n]*",
        r"(?P<incr>\*fresh(?P<version4>[0-9]+)[\s|\n]*=[\s|\n]*\(\*fresh(?P<version5>[0-9]+)\)\.wrapping_add\(1\)[\s|\n]*;)"
    ),
    replacer: ptr_incr_four_lines,
};


fn ptr_incr_four_lines(caps: &regex::Captures<'_>) -> String {
    let original = &caps[0];
    let version1 = &caps["version1"];
    let version2 = &caps["version2"];
    let version3 = &caps["version3"];
    let version4 = &caps["version4"];
    let version5 = &caps["version5"];

    if !(version1 == version3
        && version1 == version4
        && version1 == version5
        && version1 != version2)
    {
        return original.to_owned();
    }

    let expr_fresh = r"\*fresh".to_owned() + version1;

    let lhs = &caps["lhs"];
    let incr = &caps["incr"];

    let incr = regex::Regex::new(&expr_fresh)
        .unwrap()
        .replace_all(incr, lhs);

    r"let fresh".to_owned() + version2 + " = " + lhs + ";" + &incr
}
