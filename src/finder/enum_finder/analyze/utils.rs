use std::{
    collections::HashSet,
    ops::{BitOr, Deref},
};

use crate::finder::enum_finder::analyze::absyn::{FunctionName, Id, Type};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Integer(i64),
    Location(Id),
    Function(FunctionName),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct LowerBound(pub HashSet<Value>);

impl LowerBound {
    pub fn new() -> Self {
        LowerBound(HashSet::new())
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn union_with(&mut self, other: &Self) -> bool {
        let old_len = self.0.len();
        self.0.extend(other.0.iter().cloned());
        self.0.len() > old_len
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpperBound(pub Type);

impl UpperBound {
    pub fn least_upper_bound(t1: &Type, t2: &Type) -> Type {
        if t1 == t2 {
            return t1.clone();
        }
        match (t1, t2) {
            (Type::Enum(_), Type::Int) => Type::Int,
            (Type::Int, Type::Enum(_)) => Type::Int,
            (Type::Ptr(pt1), Type::Ptr(pt2)) => {
                Type::Ptr(Box::new(Self::least_upper_bound(pt1, pt2)))
            }
            _ => Type::Int,
        }
    }

    pub fn union_with(&mut self, other: &Self) -> bool {
        let new_type = Self::least_upper_bound(&self.0, &other.0);
        if self.0 != new_type {
            self.0 = new_type;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeConstraint {
    pub lower: LowerBound,
    pub upper: UpperBound,
}

impl TypeConstraint {
    pub fn new(ty: Type) -> Self {
        TypeConstraint {
            lower: LowerBound::new(),
            upper: UpperBound(ty),
        }
    }
}

impl PartialOrd for TypeConstraint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lower_subset = self.lower.is_subset(&other.lower);
        let upper_subset =
            UpperBound::least_upper_bound(&self.upper.0, &other.upper.0) == other.upper.0;

        if lower_subset && upper_subset {
            Some(std::cmp::Ordering::Less)
        } else if other.lower.is_subset(&self.lower)
            && UpperBound::least_upper_bound(&self.upper.0, &other.upper.0) == self.upper.0
        {
            Some(std::cmp::Ordering::Greater)
        } else if self == other {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

impl BitOr for &TypeConstraint {
    type Output = TypeConstraint;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut new_lower = self.lower.clone();
        new_lower.union_with(&rhs.lower);

        let new_upper_type = UpperBound::least_upper_bound(&self.upper.0, &rhs.upper.0);

        TypeConstraint {
            lower: new_lower,
            upper: UpperBound(new_upper_type),
        }
    }
}

pub struct ConstraintMap(pub std::collections::HashMap<Id, TypeConstraint>);

impl Deref for ConstraintMap {
    type Target = std::collections::HashMap<Id, TypeConstraint>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ConstraintMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
