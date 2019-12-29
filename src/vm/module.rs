use crate::vm::*;

pub fn init_module(globals: &mut Globals) -> ClassRef {
    let id = globals.get_ident_id("Module");
    let class = ClassRef::from(id, globals.object_class);
    globals.add_builtin_instance_method(class, "superclass", module_superclass);
    globals.add_builtin_instance_method(class, "constants", module_constants);
    globals.add_builtin_instance_method(class, "attr_accessor", module_attr);
    class
}

fn module_superclass(
    vm: &mut VM,
    receiver: PackedValue,
    _args: VecArray,
    _block: Option<MethodRef>,
) -> VMResult {
    match receiver.as_class() {
        Some(cref) => match cref.superclass {
            Some(superclass) => Ok(PackedValue::class(&mut vm.globals, superclass)),
            None => Ok(PackedValue::nil()),
        },
        None => Err(vm.error_internal("Illegal argument.")),
    }
}

fn module_constants(
    vm: &mut VM,
    receiver: PackedValue,
    _args: VecArray,
    _block: Option<MethodRef>,
) -> VMResult {
    match receiver.as_class() {
        Some(cref) => {
            let v: Vec<PackedValue> = cref
                .constants
                .keys()
                .map(|k| PackedValue::symbol(k.clone()))
                .collect();
            Ok(PackedValue::array(&vm.globals, ArrayRef::from(v)))
        }
        None => Err(vm.error_internal("Illegal argument.")),
    }
}

/// Built-in function "attr_accessor".
fn module_attr(
    vm: &mut VM,
    receiver: PackedValue,
    args: VecArray,
    _block: Option<MethodRef>,
) -> VMResult {
    match receiver.as_class() {
        Some(classref) => {
            for arg in args.iter() {
                if arg.is_packed_symbol() {
                    let id = arg.as_packed_symbol();
                    let info = MethodInfo::AttrReader { id };
                    let methodref = vm.globals.add_method(info);
                    vm.add_instance_method(classref, id, methodref);

                    let assign_name = vm.globals.get_ident_name(id).clone() + "=";
                    let assign_id = vm.globals.get_ident_id(assign_name);
                    let info = MethodInfo::AttrWriter { id };
                    let methodref = vm.globals.add_method(info);
                    vm.add_instance_method(classref, assign_id, methodref);
                } else {
                    return Err(vm.error_name("Each of args for attr_accessor must be a symbol."));
                }
            }
        }
        None => unreachable!("Illegal attr_accesor in non-class object."),
    }
    Ok(PackedValue::nil())
}