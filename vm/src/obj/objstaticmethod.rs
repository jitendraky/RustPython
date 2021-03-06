use super::objtype::PyClassRef;
use crate::function::OptionalArg;
use crate::pyobject::{PyClassImpl, PyContext, PyObjectRef, PyRef, PyResult, PyValue};
use crate::slots::PyBuiltinDescriptor;
use crate::vm::VirtualMachine;

#[pyclass(name = "staticmethod")]
#[derive(Clone, Debug)]
pub struct PyStaticMethod {
    pub callable: PyObjectRef,
}
pub type PyStaticMethodRef = PyRef<PyStaticMethod>;

impl PyValue for PyStaticMethod {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.staticmethod_type()
    }
}

impl PyBuiltinDescriptor for PyStaticMethod {
    fn get(
        zelf: PyRef<Self>,
        _obj: PyObjectRef,
        _cls: OptionalArg<PyObjectRef>,
        _vm: &VirtualMachine,
    ) -> PyResult {
        Ok(zelf.callable.clone())
    }
}

#[pyimpl(with(PyBuiltinDescriptor))]
impl PyStaticMethod {
    #[pyslot]
    fn tp_new(
        cls: PyClassRef,
        callable: PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<PyStaticMethodRef> {
        PyStaticMethod {
            callable: callable.clone(),
        }
        .into_ref_with_type(vm, cls)
    }
}

pub fn init(context: &PyContext) {
    PyStaticMethod::extend_class(context, &context.types.staticmethod_type);
}
