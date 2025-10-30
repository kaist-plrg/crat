use crate::finder::enum_finder::analyze::{
    absyn::{BinOp, FunctionName, Id, UnOp},
    utils::{ConstraintMap, LowerBound, Value},
};

pub fn propagate_const_lower(map: &mut ConstraintMap, lhs: &Id, val: i64) -> bool {
    let mut new_lower = LowerBound::new();
    new_lower.0.insert(Value::Integer(val));
    map.get_mut(lhs)
        .is_some_and(|c| c.lower.union_with(&new_lower))
}

pub fn propagate_copy_lower(map: &mut ConstraintMap, lhs: &Id, rhs: &Id) -> bool {
    if let Some(rhs_lower) = map.get(rhs).map(|c| c.lower.clone()) {
        map.get_mut(lhs)
            .is_some_and(|c| c.lower.union_with(&rhs_lower))
    } else {
        false
    }
}

pub fn propagate_addrof_lower(map: &mut ConstraintMap, lhs: &Id, rhs: &Id) -> bool {
    let mut new_lower = LowerBound::new();
    new_lower.0.insert(Value::Location(rhs.clone()));
    map.get_mut(lhs)
        .is_some_and(|c| c.lower.union_with(&new_lower))
}

pub fn propagate_fn_lower(map: &mut ConstraintMap, lhs: &Id, fn_name: &FunctionName) -> bool {
    let mut new_lower = LowerBound::new();
    new_lower.0.insert(Value::Function(fn_name.clone()));
    map.get_mut(lhs)
        .is_some_and(|c| c.lower.union_with(&new_lower))
}

pub fn propagate_unop_lower(map: &mut ConstraintMap, lhs: &Id, op: &UnOp, rhs: &Id) -> bool {
    let mut changed = false;
    if let Some(rhs_lower) = map.get(rhs).map(|c| c.lower.clone()) {
        let mut new_vals = LowerBound::new();
        for val in rhs_lower.0 {
            if let Value::Integer(i) = val {
                let new_int = match op {
                    UnOp::Neg => -i,
                };
                new_vals.0.insert(Value::Integer(new_int));
            }
        }
        if let Some(lhs_constraint) = map.get_mut(lhs) {
            changed = lhs_constraint.lower.union_with(&new_vals);
        }
    }
    changed
}

pub fn propagate_binop_lower(
    map: &mut ConstraintMap,
    lhs: &Id,
    op: &BinOp,
    rhs1: &Id,
    rhs2: &Id,
) -> bool {
    let mut changed = false;
    let rhs1_lower = map.get(rhs1).map(|c| c.lower.clone());
    let rhs2_lower = map.get(rhs2).map(|c| c.lower.clone());

    if let (Some(r1_lower), Some(r2_lower)) = (rhs1_lower, rhs2_lower) {
        let mut new_vals = LowerBound::new();
        for v1 in &r1_lower.0 {
            for v2 in &r2_lower.0 {
                if let (Value::Integer(i1), Value::Integer(i2)) = (v1, v2) {
                    let new_int = match op {
                        BinOp::Add => i1 + i2,
                        BinOp::Sub => i1 - i2,
                        BinOp::Mul => i1 * i2,
                        BinOp::Div if *i2 != 0 => i1 / i2,
                        _ => continue,
                    };
                    new_vals.0.insert(Value::Integer(new_int));
                }
            }
        }
        if let Some(lhs_constraint) = map.get_mut(lhs) {
            changed = lhs_constraint.lower.union_with(&new_vals);
        }
    }
    changed
}
