extern crate strum;
extern crate tuple;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate maplit;

use std::collections::HashSet;
use strum::IntoEnumIterator;
use tuple::*;

static COMPATIBILITY_CHART: [[Compatibility; 16]; 16] = [
    // INFP
    [
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
    ],
    // ENFP
    [
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
    ],
    // INFJ
    [
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
    ],
    // ENFJ
    [
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
    ],
    // INTJ
    [
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
    ],
    // ENTJ
    [
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
    ],
    // INTP
    [
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::VeryPositive,
    ],
    // ENTP
    [
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
    ],
    // ISFP
    [
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
    ],
    // ESFP
    [
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
    ],
    // ISTP
    [
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
    ],
    // ESTP
    [
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
    ],
    // ISFJ
    [
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::Negative,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
    ],
    // ESFJ
    [
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::Negative,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
    ],
    // ISTJ
    [
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::Negative,
        Compatibility::Neutral,
        Compatibility::Negative,
        Compatibility::Negative,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
    ],
    // ESTJ
    [
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::VeryNegative,
        Compatibility::Negative,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Negative,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::VeryPositive,
        Compatibility::Neutral,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
        Compatibility::Positive,
    ],
];

#[allow(dead_code)]
static FUNCTIONS: [[Function; 4]; 16] = [
    [Function::Fi, Function::Ne, Function::Si, Function::Te], // INFP
    [Function::Ne, Function::Fi, Function::Te, Function::Si], // ENFP
    [Function::Ni, Function::Fe, Function::Ti, Function::Se], // INFJ
    [Function::Fe, Function::Ni, Function::Se, Function::Ti], // ENFJ
    [Function::Ni, Function::Te, Function::Fi, Function::Se], // INTJ
    [Function::Te, Function::Ni, Function::Se, Function::Fi], // ENTJ
    [Function::Ti, Function::Ne, Function::Si, Function::Fe], // INTP
    [Function::Ne, Function::Ti, Function::Fe, Function::Si], // ENTP
    [Function::Fi, Function::Se, Function::Ni, Function::Te], // ISFP
    [Function::Se, Function::Fi, Function::Te, Function::Ni], // ESFP
    [Function::Ti, Function::Se, Function::Ni, Function::Fe], // ISTP
    [Function::Se, Function::Ti, Function::Fe, Function::Ni], // ESTP
    [Function::Si, Function::Fe, Function::Ti, Function::Ne], // ISFJ
    [Function::Fe, Function::Si, Function::Ne, Function::Ti], // ESFJ
    [Function::Si, Function::Te, Function::Fi, Function::Ne], // ISTJ
    [Function::Te, Function::Si, Function::Ne, Function::Fi], // ESTJ
];

type FunctionsGroup = (Function, Function, Function, Function);

#[derive(EnumIter, Debug, PartialEq, PartialOrd, Copy, Clone, Hash, Eq)]
pub enum Type {
    INFP,
    ENFP,
    INFJ,
    ENFJ,
    INTJ,
    ENTJ,
    INTP,
    ENTP,
    ISFP,
    ESFP,
    ISTP,
    ESTP,
    ISFJ,
    ESFJ,
    ISTJ,
    ESTJ,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Function {
    Te,
    Ti,
    Fe,
    Fi,
    Se,
    Si,
    Ne,
    Ni,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Role {
    Primary = 0,
    Auxillary = 1,
    Tertiary = 2,
    Inferior = 3,
}

#[derive(EnumIter, Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Compatibility {
    VeryPositive = 2,
    Positive = 1,
    Neutral = 0,
    Negative = -1,
    VeryNegative = -2,
}

pub struct TypeCompatibility {}

pub fn get_function(type_: Type, role: Role) -> Function {
    let functions = get_functions(type_);
    functions.get(role as usize).unwrap().clone()
}

pub fn get_functions(type_: Type) -> FunctionsGroup {
    match type_ {
        // TODO: Needs refactor
        Type::ISTJ => (Function::Si, Function::Te, Function::Fi, Function::Ne),
        Type::ISFJ => (Function::Si, Function::Fe, Function::Ti, Function::Ne),
        Type::INFJ => (Function::Ni, Function::Fe, Function::Ti, Function::Se),
        Type::INTJ => (Function::Ni, Function::Te, Function::Fi, Function::Se),
        Type::ISTP => (Function::Ti, Function::Se, Function::Ni, Function::Fe),
        Type::ISFP => (Function::Fi, Function::Se, Function::Ni, Function::Te),
        Type::INFP => (Function::Fi, Function::Ne, Function::Si, Function::Te),
        Type::INTP => (Function::Ti, Function::Ne, Function::Si, Function::Fe),
        Type::ESTP => (Function::Se, Function::Ti, Function::Fe, Function::Ni),
        Type::ESFP => (Function::Se, Function::Fi, Function::Te, Function::Ni),
        Type::ENFP => (Function::Ne, Function::Fi, Function::Te, Function::Si),
        Type::ENTP => (Function::Ne, Function::Ti, Function::Fe, Function::Si),
        Type::ESTJ => (Function::Te, Function::Si, Function::Ne, Function::Fi),
        Type::ESFJ => (Function::Fe, Function::Si, Function::Ne, Function::Ti),
        Type::ENFJ => (Function::Fe, Function::Ni, Function::Se, Function::Ti),
        Type::ENTJ => (Function::Te, Function::Ni, Function::Se, Function::Fi),
    }
}

pub fn get_type(group: FunctionsGroup) -> Option<Type> {
    match group {
        // TODO: Needs refactor
        (Function::Si, Function::Te, Function::Fi, Function::Ne) => Some(Type::ISTJ),
        (Function::Si, Function::Fe, Function::Ti, Function::Ne) => Some(Type::ISFJ),
        (Function::Ni, Function::Fe, Function::Ti, Function::Se) => Some(Type::INFJ),
        (Function::Ni, Function::Te, Function::Fi, Function::Se) => Some(Type::INTJ),
        (Function::Ti, Function::Se, Function::Ni, Function::Fe) => Some(Type::ISTP),
        (Function::Fi, Function::Se, Function::Ni, Function::Te) => Some(Type::ISFP),
        (Function::Fi, Function::Ne, Function::Si, Function::Te) => Some(Type::INFP),
        (Function::Ti, Function::Ne, Function::Si, Function::Fe) => Some(Type::INTP),
        (Function::Se, Function::Ti, Function::Fe, Function::Ni) => Some(Type::ESTP),
        (Function::Se, Function::Fi, Function::Te, Function::Ni) => Some(Type::ESFP),
        (Function::Ne, Function::Fi, Function::Te, Function::Si) => Some(Type::ENFP),
        (Function::Ne, Function::Ti, Function::Fe, Function::Si) => Some(Type::ENTP),
        (Function::Te, Function::Si, Function::Ne, Function::Fi) => Some(Type::ESTJ),
        (Function::Fe, Function::Si, Function::Ne, Function::Ti) => Some(Type::ESFJ),
        (Function::Fe, Function::Ni, Function::Se, Function::Ti) => Some(Type::ENFJ),
        (Function::Te, Function::Ni, Function::Se, Function::Fi) => Some(Type::ENTJ),
        _ => None,
    }
}

pub fn get_types(function: Function, role: Role) -> HashSet<Type> {
    let mut types: HashSet<Type> = HashSet::new();

    // TODO: Refactor because extremely inefficent
    for type_ in Type::iter() {
        if get_functions(type_).get(role as usize).unwrap().clone() == function {
            types.insert(type_);
        }
    }

    types
}

pub fn get_compatibile_types(type_: Type, compatibility: Compatibility) -> HashSet<Type> {
    match type_ {
        Type::ISTJ => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ISFJ => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::INFJ => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::INTJ => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ISTP => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ISFP => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::INFP => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::INTP => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ESTP => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ESFP => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ENFP => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ENTP => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ESTJ => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ESFJ => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ENFJ => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
        Type::ENTJ => match compatibility {
            Compatibility::VeryNegative => hashset![Type::INFP, Type::ENFP, Type::INFJ, Type::ENFJ],
            Compatibility::Negative => hashset![Type::INTJ, Type::INTP, Type::ENTP],
            Compatibility::Neutral => hashset![Type::ENTJ, Type::ISFP, Type::ISTP],
            Compatibility::Positive => hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ, Type::ESTJ],
            Compatibility::VeryPositive => hashset![Type::ESFP, Type::ESTP],
        },
    }
}

pub fn check_compatibility(first: Type, second: Type) -> Compatibility {
    COMPATIBILITY_CHART[first as usize][second as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_primary() {
        assert_eq!(get_function(Type::INTP, Role::Primary), Function::Ti);
    }

    #[test]
    fn test_get_auxilliary() {
        assert_eq!(get_function(Type::INTP, Role::Auxillary), Function::Ne);
    }

    #[test]
    fn test_get_tertiary() {
        assert_eq!(get_function(Type::INTP, Role::Tertiary), Function::Si);
    }

    #[test]
    fn test_get_inferior() {
        assert_eq!(get_function(Type::INTP, Role::Inferior), Function::Fe);
    }

    #[test]
    fn test_get_functions_correct() {
        assert_eq!(
            get_functions(Type::INTP),
            (Function::Ti, Function::Ne, Function::Si, Function::Fe)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_functions_wrong() {
        assert_eq!(
            get_functions(Type::INTP),
            (Function::Fe, Function::Si, Function::Ne, Function::Ti)
        );
    }

    #[test]
    fn test_get_types_from_primary() {
        assert_eq!(
            get_types(Function::Fe, Role::Primary),
            hashset![Type::ESFJ, Type::ENFJ]
        );
    }

    #[test]
    fn test_get_types_from_auxillary() {
        assert_eq!(
            get_types(Function::Fe, Role::Auxillary),
            hashset![Type::ISFJ, Type::INFJ]
        );
    }

    #[test]
    fn test_get_types_from_tertiary() {
        assert_eq!(
            get_types(Function::Fe, Role::Tertiary),
            hashset![Type::ESTP, Type::ENTP]
        );
    }

    #[test]
    fn test_get_types_from_inferior() {
        assert_eq!(
            get_types(Function::Fe, Role::Inferior),
            hashset![Type::ISTP, Type::INTP]
        );
    }

    #[test]
    fn test_get_type_from_functions_correct() {
        assert_eq!(
            get_type((Function::Ti, Function::Ne, Function::Si, Function::Fe)),
            Some(Type::INTP)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_type_from_functions_wrong() {
        assert_eq!(
            get_type((Function::Fe, Function::Si, Function::Ne, Function::Ti)),
            Some(Type::INTP)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_type_from_functions_not_found() {
        assert_eq!(
            get_type((Function::Fe, Function::Fi, Function::Te, Function::Ti)),
            Some(Type::INTP)
        );
    }

    #[test]
    fn test_get_compatible_types_correct() {
        assert_eq!(
            get_compatibile_types(Type::INTP, Compatibility::VeryPositive),
            hashset![Type::ENTJ, Type::ESTJ]
        );
    }

    #[test]
    #[should_panic]
    fn test_get_compatible_types_wrong() {
        assert_eq!(
            get_compatibile_types(Type::INTP, Compatibility::VeryPositive),
            hashset![Type::ISFJ, Type::ESFJ, Type::ISTJ]
        );
    }

    #[test]
    fn test_get_compatible_types_empty() {
        assert_eq!(
            get_compatibile_types(Type::INTP, Compatibility::VeryNegative),
            hashset![]
        );
    }

    #[test]
    fn test_compatibility_very_positive() {
        assert_eq!(
            check_compatibility(Type::INTP, Type::ENTJ),
            Compatibility::VeryPositive
        );
    }

    #[test]
    fn test_compatibility_positive() {
        assert_eq!(
            check_compatibility(Type::INTP, Type::INFP),
            Compatibility::Positive
        );
    }

    #[test]
    fn test_compatibility_neutral() {
        assert_eq!(
            check_compatibility(Type::INTP, Type::ISFP),
            Compatibility::Neutral
        );
    }

    #[test]
    fn test_compatibility_negative() {
        assert_eq!(
            check_compatibility(Type::INTP, Type::ISFJ),
            Compatibility::Negative
        );
    }

    #[test]
    fn test_compatibility_very_negative() {
        assert_eq!(
            check_compatibility(Type::ISFP, Type::INFP),
            Compatibility::VeryNegative
        );
    }
}
