pub const PATTERNS: &[Pattern] = &[
    ASSIGN_VALUE_AS_ASSIGNER4,
    ASSIGN_VALUE_AS_ASSIGNER3,
    ASSIGN_VALUE_AS_ASSIGNER1,
    ASSIGN_VALUE_AS_ASSIGNER2,
    ASSIGN_VALUE_IN_CONDITION,
    ASSIGN_TWO_LINES,
    PTR_INCR_FOUR_LINES,
];

pub struct Pattern {
    pub pattern: &'static str,
    pub replacer: for<'t> fn(&regex::Captures<'t>) -> String,
}


const ASSIGN_VALUE_AS_ASSIGNER4: Pattern = Pattern {
    pattern: concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<w>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*(?P<val>[^;]+);[\s|\n]*",
        r"let ref mut fresh(?P<version3>[0-9]+)[\s|\n]*=[\s|\n]*(?P<z>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version4>[0-9]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version5>[0-9]+);[\s|\n]*",
        r"let ref mut fresh(?P<version6>[0-9]+)[\s|\n]*=[\s|\n]*(?P<y>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version7>[0-9]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version8>[0-9]+);[\s|\n]*",
        r"let ref mut fresh(?P<version9>[0-9]+)[\s|\n]*=[\s|\n]*(?P<x>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version10>[0-9]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version11>[0-9]+);",
    ),
    replacer: assign_value_as_assigner4,
};

fn assign_value_as_assigner4(caps: &regex::Captures<'_>) -> String {

    let original = &caps[0];
    let version1 = &caps["version1"];
    let version2 = &caps["version2"];
    let version3 = &caps["version3"];
    let version4 = &caps["version4"];
    let version5 = &caps["version5"];
    let version6 = &caps["version6"];
    let version7 = &caps["version7"];
    let version8 = &caps["version8"];
    let version9 = &caps["version9"];
    let version10 = &caps["version10"];
    let version11 = &caps["version11"];


    if !(version1 == version2
        && version1 != version3
        && version3 == version4
        && version1 == version5
        && version1 != version6
        && version3 != version6
        && version6 == version7
        && version3 == version8
        && version1 != version9
        && version3 != version9
        && version6 != version9
        && version9 == version10
        && version6 == version11)
    {
        return original.to_owned();
    }

    let x = &caps["x"];
    let y = &caps["y"];
    let z = &caps["z"];
    let w = &caps["w"];
    let val = &caps["val"];

    w.to_owned() + " = " + val + "; " + z + " = " + w + "; " + y + " = " + z + "; " + x + " = " + y + ";"
}

/// ```c
/// x = y = z = value;
/// ```
/// is translated to
/// ```rust
/// let ref mut fresh1 = z;
/// *fresh1 = val;
/// let ref mut fresh2 = y;
/// *fresh2 = *fresh1;
/// let ref mut fresh3 = x;
/// *fresh3 = *fresh2;
/// ```
const ASSIGN_VALUE_AS_ASSIGNER3: Pattern = Pattern {
    pattern: concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<z>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*(?P<val>[^;]+);[\s|\n]*",
        r"let ref mut fresh(?P<version3>[0-9]+)[\s|\n]*=[\s|\n]*(?P<y>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version4>[0-9]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version5>[0-9]+);[\s|\n]*",
        r"let ref mut fresh(?P<version6>[0-9]+)[\s|\n]*=[\s|\n]*(?P<x>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version7>[0-9]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version8>[0-9]+);",
    ),
    replacer: assign_value_as_assigner3,
};

fn assign_value_as_assigner3(caps: &regex::Captures<'_>) -> String {

    let original = &caps[0];
    let version1 = &caps["version1"];
    let version2 = &caps["version2"];
    let version3 = &caps["version3"];
    let version4 = &caps["version4"];
    let version5 = &caps["version5"];
    let version6 = &caps["version6"];
    let version7 = &caps["version7"];
    let version8 = &caps["version8"];


    if !(version1 == version2
        && version1 != version3
        && version3 == version4
        && version1 == version5
        && version1 != version6
        && version3 != version6
        && version6 == version7
        && version3 == version8)
    {
        return original.to_owned();
    }

    let x = &caps["x"];
    let y = &caps["y"];
    let z = &caps["z"];
    let val = &caps["val"];

    z.to_owned() + " = " + val + "; " + y + " = " + z + "; " + x + " = " + y + ";"
}

/// ```c
/// x = y = value;
/// ```
/// is translated to
/// ```rust
/// let ref mut fresh1 = rhs;
/// *fresh1 = val;
/// let ref mut fresh2 = lhs;
/// *fresh2 = *fresh1;
/// ```
const ASSIGN_VALUE_AS_ASSIGNER1: Pattern = Pattern {
    pattern: concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs1>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*(?P<rhs>[^;]+);[\s|\n]*",
        r"let ref mut fresh(?P<version3>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs2>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version4>[0-9]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version5>[0-9]+)[\s|\n]*;",
    ),
    replacer: assign_value_as_assigner1,
};

fn assign_value_as_assigner1(caps: &regex::Captures<'_>) -> String {
    let original = &caps[0];
    let version1 = &caps["version1"];
    let version2 = &caps["version2"];
    let version3 = &caps["version3"];
    let version4 = &caps["version4"];
    let version5 = &caps["version5"];


    if !(version1 == version2
        && version1 != version3
        && version3 == version4
        && version1 == version5)
    {
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
/// x = y = value;
/// ```
/// is translated to
/// ```rust
/// let ref mut fresh1 = rhs;
/// *fresh1 = val;
/// lhs = *fresh1;
/// ```
const ASSIGN_VALUE_AS_ASSIGNER2: Pattern = Pattern {
    pattern: concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs1>[^;]+);[\s|\n]*",
        r"\*fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*(?P<rhs>[^;]+);[\s|\n]*",
        r"(?P<lhs2>[^;]+)[\s|\n]*=[\s|\n]*\*fresh(?P<version3>[0-9]+)[\s|\n]*;",
    ),
    replacer: assign_value_as_assigner2,
};

fn assign_value_as_assigner2(caps: &regex::Captures<'_>) -> String {
    let original = &caps[0];
    let version1 = &caps["version1"];
    let version2 = &caps["version2"];
    let version3 = &caps["version3"];

    if !(version1 == version2 && version1 == version3) {
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
const ASSIGN_VALUE_IN_CONDITION: Pattern = Pattern {
    pattern: concat!(
        r"let ref mut fresh(?P<version1>[0-9]+)[\s|\n]*=[\s|\n]*(?P<lhs>[^;]+);",
        r"[\s|\n]*\*fresh(?P<version2>[0-9]+)[\s|\n]*=[\s|\n]*(?P<rhs>[^;]+);",
        r"(?P<line3>[\s|\n]*if[\s|\n]*\(\*fresh(?P<version3>[0-9]+)\).is_null\(\))"
    ),
    replacer: assign_value_in_condition,
};

fn assign_value_in_condition(caps: &regex::Captures<'_>) -> String {
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
        r"[\s|\n]*\*fresh(?P<version2>[0-9]+)[\s|\n]*(?P<assignop>[&|\^|\+|-|\*|/|%|\|]*=)[\s|\n]*(?P<rhs>[^;]+);"
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
    let assign_op = &caps["assignop"];

    let rhs = regex::Regex::new(&expr_fresh)
        .unwrap()
        .replace_all(rhs, lhs);

    lhs.to_owned() + " " + assign_op + " " + &rhs + ";"
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
