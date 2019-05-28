type FunctionsGroup = (Function, Function, Function, Function);

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Type {
    ISTJ,
    ISFJ,
    INFJ,
    INTJ,
    ISTP,
    ISFP,
    INFP,
    INTP,
    ESTP,
    ESFP,
    ENFP,
    ENTP,
    ESTJ,
    ESFJ,
    ENFJ,
    ENTJ,
}

#[derive(Debug, PartialEq, PartialOrd)]
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

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Role {
    Primary = 0,
    Auxillary = 1,
    Tertiary = 2,
    Inferior = 3,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Compatibility {
    VeryPositive = 2,
    Positive = 1,
    Neutral = 0,
    Negative = -1,
    VeryNegative = -2,
}

pub fn get_function(type_: Type, role: Role) -> Function {
    Function::Te
}

pub fn get_functions(type_: Type) -> FunctionsGroup {
    (Function::Fe, Function::Fe, Function::Fe, Function::Fe)
}

pub fn get_type(group: FunctionsGroup) -> Option<Type> {
    Some(Type::INTP)
}

pub fn get_types(function: Function, role: Role) -> Vec<Type> {
    Vec::new()
}

pub fn check_compatibility(first: Type, second: Type) -> Compatibility {
    Compatibility::Neutral
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
            vec![Type::ESFJ, Type::ENFJ]
        );
    }

    #[test]
    fn test_get_types_from_auxillary() {
        assert_eq!(
            get_types(Function::Fe, Role::Auxillary),
            vec![Type::ISFJ, Type::INFJ]
        );
    }

    #[test]
    fn test_get_types_from_tertiary() {
        assert_eq!(
            get_types(Function::Fe, Role::Tertiary),
            vec![Type::ESTP, Type::ENTP]
        );
    }

    #[test]
    fn test_get_types_from_inferior() {
        assert_eq!(
            get_types(Function::Fe, Role::Inferior),
            vec![Type::ISTP, Type::INTP]
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
            Compatibility::VeryPositive
        );
    }

    #[test]
    fn test_compatibility_neutral() {
        assert_eq!(
            check_compatibility(Type::INTP, Type::ISFP),
            Compatibility::VeryPositive
        );
    }

    #[test]
    fn test_compatibility_negative() {
        assert_eq!(
            check_compatibility(Type::INTP, Type::ISFJ),
            Compatibility::VeryPositive
        );
    }

    #[test]
    fn test_compatibility_very_negative() {
        assert_eq!(
            check_compatibility(Type::ISFP, Type::INFP),
            Compatibility::VeryPositive
        );
    }
}
