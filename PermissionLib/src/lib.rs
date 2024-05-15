use interoptopus::{ffi_function, ffi_type, function, Inventory, InventoryBuilder};
use interoptopus::patterns::string::AsciiPointer;

use cedar_policy::PrincipalConstraint::{Any, Eq, In, Is, IsIn};
use cedar_policy::{
    Authorizer, Context, Decision, Entities, Entity, EntityId, EntityTypeName, EntityUid, Policy,
    PolicyId, PolicySet, Request, Response, RestrictedExpression, Schema, SlotId, Template,
    ValidationMode, ValidationResult, Validator,
};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;


// /// A simple type in our FFI layer.
// #[ffi_type]
// #[repr(C)]
// pub struct Vec2 {
//     pub x: f32,
//     pub y: f32,
// }

// /// Function using the type.
// #[ffi_function]
// #[no_mangle]
// pub extern "C" fn my_function(input: Vec2) -> Vec2 {
//     Vec2 { x: input.x + 1.0, y: input.y + 2.0 }   
// }

/// Function using the type.
#[ffi_function]
#[no_mangle]
pub extern "C" fn init(policies: &AsciiPointer, schema: &AsciiPointer) -> bool {
    let p = policies.as_str().unwrap();
    let sc = schema.as_str().unwrap();

    let _schema = Schema::from_str_natural(sc).unwrap().0;
    let _policies = PolicySet::from_str(p).unwrap();

    true
}

/// Function using the type.
#[ffi_function]
#[no_mangle]
pub extern "C" fn clean() -> bool {
    true
}

/// Function using the type.
#[ffi_function]
#[no_mangle]
pub extern "C" fn check(
    policies: &AsciiPointer, 
    schema: &AsciiPointer,
    principal: &AsciiPointer,
    action: &AsciiPointer,
    resource: &AsciiPointer,
    context: &AsciiPointer, 
    entities: &AsciiPointer) -> bool 
{
    let p = policies.as_str().unwrap();
    let sc = schema.as_str().unwrap();

    let sch = Schema::from_str_natural(sc).unwrap().0;
    let policies = PolicySet::from_str(p).unwrap();

    let pr = principal.as_str().unwrap();
    let pri = EntityUid::from_str(pr).ok();
    let a = action.as_str().unwrap();
    let ac = EntityUid::from_str(a).ok();
    
    let a_eid = EntityId::from_str("searchProduct").unwrap(); // does not go through the parser
    let a_name: EntityTypeName = EntityTypeName::from_str("OnlineStore::Action").unwrap(); // through parse_name(s)
    let a2 = EntityUid::from_type_name_and_id(a_name, a_eid);

    let re = None;
    let ct = context.as_str().unwrap();
    let ctx = Context::from_json_str(ct, None).unwrap();
    let request = Request::new(pri, Some(a2), re, ctx, None);
    
    let authorizer = Authorizer::new();
    let ans = authorizer.is_authorized(&request.unwrap(), &policies, &Entities::empty());

    ans.decision() == Decision::Allow
}

// This will create a function `my_inventory` which can produce
// an abstract FFI representation (called `Library`) for this crate.
pub fn my_inventory() -> Inventory {
    {
        InventoryBuilder::new()
        .register(function!(init))
        .register(function!(clean))
        .register(function!(check))
        .inventory()
    }
}
