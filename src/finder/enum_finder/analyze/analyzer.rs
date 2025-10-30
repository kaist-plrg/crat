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
        let changed = match stmt {
            Statement::Assign(lhs, rvalue) => process_rvalue(lhs, rvalue, map),
            Statement::DerefAssign(lhs_ptr, rhs) => {
                let mut changed = false;
                if let Some(lhs_lower) = map.get(lhs_ptr).map(|c| c.lower.clone()) {
                    for val in lhs_lower.0 {
                        if let Value::Location(loc_id) = val
                            && let Some(rhs_constraint) = map.get(rhs).cloned()
                            && let Some(loc_constraint) = map.get_mut(&loc_id)
                        {
                            let c1 = loc_constraint.lower.union_with(&rhs_constraint.lower);
                            let c2 = loc_constraint.upper.union_with(&rhs_constraint.upper);
                            if c1 || c2 {
                                changed = true;
                                changed_vars.insert(loc_id);
                            }
                        }
                    }
                }
                if upper::propagate_deref_assign_upper(map, lhs_ptr, rhs) {
                    changed = true;
                    changed_vars.insert(rhs.clone());
                }
                changed
            }
            _ => false,
        };
        if changed && let Statement::Assign(lhs, _) = stmt {
            changed_vars.insert(lhs.clone());
        }
    }

    if let Terminator::Call(_lhs, _, _args, _) = &block.terminator {}

    changed_vars
}

fn process_rvalue(lhs: &Id, rvalue: &RValue, map: &mut ConstraintMap) -> bool {
    match rvalue {
        RValue::UseInt(n) => lower::propagate_const_lower(map, lhs, *n),
        RValue::Copy(rhs) => {
            let c1 = lower::propagate_copy_lower(map, lhs, rhs);
            let c2 = upper::propagate_copy_upper(map, lhs, rhs);
            c1 || c2
        }
        RValue::UnaryOp(op, rhs) => lower::propagate_unop_lower(map, lhs, op, rhs),
        RValue::BinaryOp(op, r1, r2) => lower::propagate_binop_lower(map, lhs, op, r1, r2),
        RValue::AddrOf(rhs) => {
            let c1 = lower::propagate_addrof_lower(map, lhs, rhs);
            let c2 = upper::propagate_addrof_upper(map, lhs, rhs);
            c1 || c2
        }
        RValue::Deref(rhs) => {
            let mut changed = false;
            if let Some(rhs_lower) = map.get(rhs).map(|c| c.lower.clone()) {
                for val in rhs_lower.0 {
                    if let Value::Location(loc_id) = val
                        && let Some(loc_constraint) = map.get(&loc_id).cloned()
                        && let Some(lhs_constraint) = map.get_mut(lhs)
                    {
                        let c1 = lhs_constraint.lower.union_with(&loc_constraint.lower);
                        let c2 = lhs_constraint.upper.union_with(&loc_constraint.upper);
                        if c1 || c2 {
                            changed = true;
                        }
                    }
                }
            }
            if upper::propagate_deref_upper(map, lhs, rhs) {
                changed = true;
            }
            changed
        }
        RValue::UseFn(fn_name) => lower::propagate_fn_lower(map, lhs, fn_name),
        _ => false,
    }
}

pub fn analyze_function(func: &Function) -> ConstraintMap {
    let mut map = initialize_constraints(func);
    solve(func, &mut map);
    map
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::finder::enum_finder::analyze::{absyn::*, utils::*};

    #[test]
    fn test_simple_case() {
        let mut locals = HashMap::new();
        let enum_type = Type::Enum("E".to_string());
        locals.insert("x".to_string(), enum_type.clone());
        locals.insert("y".to_string(), Type::Int);

        let block0 = BasicBlock {
            statements: vec![],
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
            statements: vec![],
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
            statements: vec![],
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
