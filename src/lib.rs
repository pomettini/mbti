#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate maplit;

use std::collections::HashSet;
use strum::IntoEnumIterator;

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

impl Type {
    fn at_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Type::INFP),
            1 => Some(Type::ENFP),
            2 => Some(Type::INFJ),
            3 => Some(Type::ENFJ),
            4 => Some(Type::INTJ),
            5 => Some(Type::ENTJ),
            6 => Some(Type::INTP),
            7 => Some(Type::ENTP),
            8 => Some(Type::ISFP),
            9 => Some(Type::ESFP),
            10 => Some(Type::ISTP),
            11 => Some(Type::ESTP),
            12 => Some(Type::ISFJ),
            13 => Some(Type::ESFJ),
            14 => Some(Type::ISTJ),
            15 => Some(Type::ESTJ),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
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
    VeryPositive,
    Positive,
    Neutral,
    Negative,
    VeryNegative,
}

pub fn get_function(type_: Type, role: Role) -> Function {
    FUNCTIONS[type_ as usize][role as usize]
}

pub fn get_functions_from_type(type_: Type) -> Vec<Function> {
    vec![
        FUNCTIONS[type_ as usize][0],
        FUNCTIONS[type_ as usize][1],
        FUNCTIONS[type_ as usize][2],
        FUNCTIONS[type_ as usize][3],
    ]
}

pub fn get_type_from_functions(input: &[Function]) -> Option<Type> {
    for (iterator, function) in FUNCTIONS.iter().enumerate() {
        if input == function {
            return Type::at_index(iterator);
        }
    }

    None
}

pub fn get_types_from_function_role(function: Function, role: Role) -> HashSet<Type> {
    let mut types: HashSet<Type> = HashSet::new();

    // TODO: Refactor because inefficent
    for type_ in Type::iter() {
        if get_functions_from_type(type_)[role as usize] == function {
            types.insert(type_);
        }
    }

    types
}

pub fn get_compatibile_types(type_: Type, compatibility: Compatibility) -> HashSet<Type> {
    let mut set: HashSet<Type> = HashSet::new();

    for (iterator, types) in COMPATIBILITY_CHART[type_ as usize].iter().enumerate() {
        if types == &compatibility {
            set.insert(Type::at_index(iterator).unwrap());
        }
    }

    set
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
        assert_eq!(get_function(Type::ENFP, Role::Auxillary), Function::Fi);
    }

    #[test]
    fn test_get_tertiary() {
        assert_eq!(get_function(Type::INFJ, Role::Tertiary), Function::Ti);
    }

    #[test]
    fn test_get_inferior() {
        assert_eq!(get_function(Type::ENFJ, Role::Inferior), Function::Ti);
    }

    #[test]
    fn test_get_functions_correct() {
        assert_eq!(
            get_functions_from_type(Type::INTP),
            vec![Function::Ti, Function::Ne, Function::Si, Function::Fe]
        );
    }

    #[test]
    #[should_panic]
    fn test_get_functions_wrong() {
        assert_eq!(
            get_functions_from_type(Type::ENTP),
            vec![Function::Fe, Function::Si, Function::Ne, Function::Ti]
        );
    }

    #[test]
    fn test_get_types_from_primary() {
        assert_eq!(
            get_types_from_function_role(Function::Fe, Role::Primary),
            hashset![Type::ESFJ, Type::ENFJ]
        );
    }

    #[test]
    fn test_get_types_from_auxillary() {
        assert_eq!(
            get_types_from_function_role(Function::Fe, Role::Auxillary),
            hashset![Type::ISFJ, Type::INFJ]
        );
    }

    #[test]
    fn test_get_types_from_tertiary() {
        assert_eq!(
            get_types_from_function_role(Function::Fe, Role::Tertiary),
            hashset![Type::ESTP, Type::ENTP]
        );
    }

    #[test]
    fn test_get_types_from_inferior() {
        assert_eq!(
            get_types_from_function_role(Function::Fe, Role::Inferior),
            hashset![Type::ISTP, Type::INTP]
        );
    }

    #[test]
    fn test_get_type_from_functions_correct() {
        assert_eq!(
            get_type_from_functions(&[Function::Ti, Function::Ne, Function::Si, Function::Fe]),
            Some(Type::INTP)
        );
    }

    #[test]
    #[should_panic]
    fn test_get_type_from_functions_wrong() {
        assert_eq!(
            get_type_from_functions(&[Function::Fe, Function::Si, Function::Ne, Function::Ti]),
            Some(Type::INTP)
        );
    }

    #[test]
    fn test_get_type_from_functions_not_found() {
        assert_eq!(
            get_type_from_functions(&[Function::Fe, Function::Fi, Function::Te, Function::Ti]),
            None
        );
    }

    #[test]
    fn test_get_type_from_functions_incorrect_number() {
        assert_eq!(
            get_type_from_functions(&[Function::Fe, Function::Fi, Function::Te]),
            None
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
