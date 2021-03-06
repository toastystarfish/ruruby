use crate::*;

#[derive(Debug, Clone)]
pub struct ProcInfo {
    pub context: ContextRef,
}

impl ProcInfo {
    pub fn new(context: ContextRef) -> Self {
        ProcInfo { context }
    }
}

pub type ProcRef = Ref<ProcInfo>;

impl ProcRef {
    pub fn from(context: ContextRef) -> Self {
        ProcRef::new(ProcInfo::new(context))
    }
}

pub fn init_proc(globals: &mut Globals) -> Value {
    let proc_id = globals.get_ident_id("Proc");
    let class = ClassRef::from(proc_id, globals.builtins.object);
    let obj = Value::class(globals, class);
    globals.add_builtin_instance_method(class, "call", proc_call);
    globals.add_builtin_class_method(obj, "new", proc_new);
    obj
}

// Class methods

fn proc_new(vm: &mut VM, _: Value, args: &Args) -> VMResult {
    let method = vm.expect_block(args.block)?;
    let procobj = vm.create_proc(method)?;
    Ok(procobj)
}

// Instance methods

fn proc_call(vm: &mut VM, self_val: Value, args: &Args) -> VMResult {
    let pref = match self_val.as_proc() {
        Some(pref) => pref,
        None => return Err(vm.error_unimplemented("Expected Proc object.")),
    };
    let context = Context::from_args(
        vm,
        self_val,
        pref.context.iseq_ref,
        args,
        pref.context.outer,
    )?;
    let res = vm.run_context(ContextRef::from_local(&context))?;
    Ok(res)
}
