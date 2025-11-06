use crate::finder::enum_finder::analyze::{
    absyn::{Id, Type},
    utils::{ConstraintMap, UpperBound},
};

pub fn propagate_copy_upper(map: &mut ConstraintMap, lhs: &Id, rhs: &Id) -> bool {
    if let Some(lhs_upper) = map.get(lhs).map(|c| c.upper.clone()) {
        map.get_mut(rhs)
            .is_some_and(|c| c.upper.intersect_with(&lhs_upper))
    } else {
        false
    }
}

pub fn propagate_addrof_upper(map: &mut ConstraintMap, lhs: &Id, rhs: &Id) -> bool {
    let mut changed = false;
    if let Some(Type::Ptr(pointee_type)) = map.get(lhs).map(|c| c.upper.0.clone()) {
        let new_upper_for_rhs = UpperBound(*pointee_type);
        if let Some(rhs_constraint) = map.get_mut(rhs) {
            changed = rhs_constraint.upper.intersect_with(&new_upper_for_rhs);
        }
    }
    changed
}

pub fn propagate_deref_upper(map: &mut ConstraintMap, lhs: &Id, rhs: &Id) -> bool {
    let mut changed = false;
    if let Some(lhs_upper) = map.get(lhs).map(|c| c.upper.clone()) {
        let new_upper_for_rhs = UpperBound(Type::Ptr(Box::new(lhs_upper.0)));
        if let Some(rhs_constraint) = map.get_mut(rhs) {
            changed = rhs_constraint.upper.intersect_with(&new_upper_for_rhs);
        }
    }
    changed
}

pub fn propagate_deref_assign_upper(map: &mut ConstraintMap, lhs_ptr: &Id, rhs: &Id) -> bool {
    let mut changed = false;
    if let Some(Type::Ptr(pointee_type)) = map.get(lhs_ptr).map(|c| c.upper.0.clone()) {
        let new_upper_for_rhs = UpperBound(*pointee_type);
        if let Some(rhs_constraint) = map.get_mut(rhs) {
            changed = rhs_constraint.upper.intersect_with(&new_upper_for_rhs);
        }
    }
    changed
}
