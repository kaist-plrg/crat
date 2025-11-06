use std::collections::{HashMap, HashSet, VecDeque};

use crate::finder::enum_finder::analyze::{
    absyn::{BasicBlock, Function, Id, RValue, Statement, Terminator},
    lower, upper,
    utils::{ConstraintMap, TypeConstraint, Value},
};

fn initialize_constraints(func: &Function) -> ConstraintMap {
    let mut map = HashMap::new();
    for (id, ty) in &func.locals {
        map.insert(id.clone(), TypeConstraint::new(ty.clone()));
    }
    ConstraintMap(map)
}

fn solve(func: &Function, map: &mut ConstraintMap) {
    let mut worklist: VecDeque<Id> = map.keys().cloned().collect();
    let mut worklist_set: HashSet<Id> = map.keys().cloned().collect();

    while let Some(id) = worklist.pop_front() {
        worklist_set.remove(&id);

        for block in func.blocks.values() {
            let changed_vars = process_block(block, map);
            for var in changed_vars {
                if !worklist_set.contains(&var) {
                    worklist.push_back(var.clone());
                    worklist_set.insert(var);
                }
            }
        }
    }
}

fn process_block(block: &BasicBlock, map: &mut ConstraintMap) -> HashSet<Id> {
    let mut changed_vars = HashSet::new();

    for stmt in &block.statements {
        match stmt {
            Statement::Assign(lhs, rvalue) => {
                let new_changes = process_rvalue(lhs, rvalue, map);
                changed_vars.extend(new_changes);
            }
            Statement::DerefAssign(lhs_ptr, rhs) => {
                if let Some(lhs_lower) = map.get(lhs_ptr).map(|c| c.lower.clone()) {
                    for val in lhs_lower.0 {
                        if let Value::Location(loc_id) = val
                            && let Some(rhs_constraint) = map.get(rhs).cloned()
                            && let Some(loc_constraint) = map.get_mut(&loc_id)
                        {
                            let c1 = loc_constraint.lower.union_with(&rhs_constraint.lower);

                            let c2 = loc_constraint.upper.intersect_with(&rhs_constraint.upper);
                            if c1 || c2 {
                                changed_vars.insert(loc_id);
                            }
                        }
                    }
                }
                if upper::propagate_deref_assign_upper(map, lhs_ptr, rhs) {
                    changed_vars.insert(rhs.clone());
                }
            }
            _ => {}
        };
    }

    if let Terminator::Call(_lhs, _, _args, _) = &block.terminator {}

    changed_vars
}

fn process_rvalue(lhs: &Id, rvalue: &RValue, map: &mut ConstraintMap) -> HashSet<Id> {
    let mut changed_vars = HashSet::new();
    let mut lhs_changed = false;

    match rvalue {
        RValue::UseInt(n) => {
            lhs_changed = lower::propagate_const_lower(map, lhs, *n);
        }
        RValue::Copy(rhs) => {
            lhs_changed = lower::propagate_copy_lower(map, lhs, rhs);
            if upper::propagate_copy_upper(map, lhs, rhs) {
                changed_vars.insert(rhs.clone());
            }
        }
        RValue::UnaryOp(op, rhs) => {
            lhs_changed = lower::propagate_unop_lower(map, lhs, op, rhs);
        }
        RValue::BinaryOp(op, r1, r2) => {
            lhs_changed = lower::propagate_binop_lower(map, lhs, op, r1, r2);
        }
        RValue::AddrOf(rhs) => {
            lhs_changed = lower::propagate_addrof_lower(map, lhs, rhs);
            if upper::propagate_addrof_upper(map, lhs, rhs) {
                changed_vars.insert(rhs.clone());
            }
        }
        RValue::Deref(rhs) => {
            let mut lhs_lower_changed = false;

            if let Some(rhs_lower) = map.get(rhs).map(|c| c.lower.clone()) {
                for val in rhs_lower.0 {
                    if let Value::Location(loc_id) = val {
                        let lhs_upper_clone = map.get(lhs).map(|c| c.upper.clone());
                        let loc_lower_clone = map.get(&loc_id).map(|c| c.lower.clone());

                        if let (Some(loc_lower), Some(lhs_constraint_mut)) =
                            (loc_lower_clone, map.get_mut(lhs))
                            && lhs_constraint_mut.lower.union_with(&loc_lower)
                        {
                            lhs_lower_changed = true;
                        }

                        if let (Some(lhs_upper), Some(loc_constraint_mut)) =
                            (lhs_upper_clone, map.get_mut(&loc_id))
                            && loc_constraint_mut.upper.intersect_with(&lhs_upper)
                        {
                            changed_vars.insert(loc_id.clone());
                        }
                    }
                }
            }
            lhs_changed = lhs_lower_changed;

            if upper::propagate_deref_upper(map, lhs, rhs) {
                changed_vars.insert(rhs.clone());
            }
        }
        RValue::UseFn(fn_name) => {
            lhs_changed = lower::propagate_fn_lower(map, lhs, fn_name);
        }
        _ => {}
    }

    if lhs_changed {
        changed_vars.insert(lhs.clone());
    }
    changed_vars
}

pub fn analyze_function(func: &Function) -> ConstraintMap {
    let mut map = initialize_constraints(func);
    solve(func, &mut map);
    map
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;
    use crate::finder::enum_finder::analyze::{absyn::*, utils::*};

    #[test]
    fn test_simple_case() {
        let mut locals = HashMap::new();
        let enum_type = Type::Enum("E".to_string());
        locals.insert("x".to_string(), enum_type.clone());
        locals.insert("y".to_string(), Type::Int);

        let block0 = BasicBlock {
            statements: vec![
                Statement::Assign("y".to_string(), RValue::UseInt(1)),
                Statement::Assign("x".to_string(), RValue::Copy("y".to_string())),
            ],
            terminator: Terminator::Return,
        };
        let mut blocks = HashMap::new();
        blocks.insert(0, block0);

        let func = Function {
            name: "test_fn".to_string(),
            blocks,
            locals,
        };

        let result_map = analyze_function(&func);

        let x_constraint = result_map.get("x").unwrap();
        assert_eq!(x_constraint.upper.0, enum_type);
        let expected_x_lower: HashSet<Value> = [Value::Integer(1)].iter().cloned().collect();
        assert_eq!(x_constraint.lower.0, expected_x_lower);

        let y_constraint = result_map.get("y").unwrap();
        assert_eq!(y_constraint.upper.0, enum_type);
        let expected_y_lower: HashSet<Value> = [Value::Integer(1)].iter().cloned().collect();
        assert_eq!(y_constraint.lower.0, expected_y_lower);
    }

    #[test]
    fn test_unary_op_case() {
        let mut locals = HashMap::new();
        let enum_type = Type::Enum("E".to_string());
        locals.insert("x".to_string(), enum_type.clone());
        locals.insert("y".to_string(), Type::Int);

        let block0 = BasicBlock {
            statements: vec![
                Statement::Assign("y".to_string(), RValue::UseInt(-1)),
                Statement::Assign("x".to_string(), RValue::UnaryOp(UnOp::Neg, "y".to_string())),
            ],
            terminator: Terminator::Return,
        };
        let mut blocks = HashMap::new();
        blocks.insert(0, block0);

        let func = Function {
            name: "test_fn".to_string(),
            blocks,
            locals,
        };

        let result_map = analyze_function(&func);

        let x_constraint = result_map.get("x").unwrap();
        assert_eq!(x_constraint.upper.0, enum_type);
        let expected_x_lower: HashSet<Value> = [Value::Integer(1)].iter().cloned().collect();
        assert_eq!(x_constraint.lower.0, expected_x_lower);

        let y_constraint = result_map.get("y").unwrap();
        assert_eq!(y_constraint.upper.0, Type::Int);
        let expected_y_lower: HashSet<Value> = [Value::Integer(-1)].iter().cloned().collect();
        assert_eq!(y_constraint.lower.0, expected_y_lower);
    }

    #[test]
    fn test_pointer_case() {
        let mut locals = HashMap::new();
        let enum_type = Type::Enum("E".to_string());
        let ptr_enum_type = Type::Ptr(Box::new(enum_type.clone()));

        locals.insert("x".to_string(), Type::Ptr(Box::new(Type::Int)));
        locals.insert("y".to_string(), Type::Int);
        locals.insert("z".to_string(), enum_type.clone());

        let block0 = BasicBlock {
            statements: vec![
                Statement::Assign("y".to_string(), RValue::UseInt(1)),
                Statement::Assign("x".to_string(), RValue::AddrOf("y".to_string())),
                Statement::Assign("z".to_string(), RValue::Deref("x".to_string())),
            ],
            terminator: Terminator::Return,
        };
        let mut blocks = HashMap::new();
        blocks.insert(0, block0);

        let func = Function {
            name: "test_fn".to_string(),
            blocks,
            locals,
        };

        let result_map = analyze_function(&func);

        let x_constraint = result_map.get("x").unwrap();
        assert_eq!(x_constraint.upper.0, ptr_enum_type);
        let expected_x_lower: HashSet<Value> =
            [Value::Location("y".to_string())].iter().cloned().collect();
        assert_eq!(x_constraint.lower.0, expected_x_lower);

        let y_constraint = result_map.get("y").unwrap();
        assert_eq!(y_constraint.upper.0, enum_type);
        let expected_y_lower: HashSet<Value> = [Value::Integer(1)].iter().cloned().collect();
        assert_eq!(y_constraint.lower.0, expected_y_lower);

        let z_constraint = result_map.get("z").unwrap();
        assert_eq!(z_constraint.upper.0, enum_type);
        let expected_z_lower: HashSet<Value> = [Value::Integer(1)].iter().cloned().collect();
        assert_eq!(z_constraint.lower.0, expected_z_lower);
    }
}
