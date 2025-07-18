use ::libc;
extern "C" {
    pub type PKVM;
    pub type PkHandle;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn pkReleaseHandle(vm: *mut PKVM, handle: *mut PkHandle);
    fn pkNewModule(vm: *mut PKVM, name: *const libc::c_char) -> *mut PkHandle;
    fn pkRegisterModule(vm: *mut PKVM, module: *mut PkHandle);
    fn pkModuleAddFunction(
        vm: *mut PKVM,
        module: *mut PkHandle,
        name: *const libc::c_char,
        fptr: pkNativeFn,
        arity: libc::c_int,
        docstring: *const libc::c_char,
    );
    fn pkSetRuntimeError(vm: *mut PKVM, message: *const libc::c_char);
    fn pkValidateSlotNumber(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut libc::c_double,
    ) -> bool;
    fn pkReserveSlots(vm: *mut PKVM, count: libc::c_int);
    fn pkSetSlotNumber(vm: *mut PKVM, index: libc::c_int, value: libc::c_double);
    fn pkSetSlotHandle(vm: *mut PKVM, index: libc::c_int, handle: *mut PkHandle);
    fn pkSetAttribute(
        vm: *mut PKVM,
        instance: libc::c_int,
        name: *const libc::c_char,
        value: libc::c_int,
    ) -> bool;
}
pub type pkNativeFn = Option::<unsafe extern "C" fn(*mut PKVM) -> ()>;
static mut _pk_doc_stdMathFloor: *const libc::c_char = b"math.floor(value:Numberber) -> Numberber\n\nReturn the floor value.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathFloor(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, floor(num));
}
static mut _pk_doc_stdMathCeil: *const libc::c_char = b"math.ceil(value:Number) -> Number\n\nReturns the ceiling value.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathCeil(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, ceil(num));
}
static mut _pk_doc_stdMathPow: *const libc::c_char = b"math.pow(a:Number, b:Number) -> Number\n\nReturns the power 'b' of 'a' similler to a**b.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathPow(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    let mut ex: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    if !pkValidateSlotNumber(vm, 2 as libc::c_int, &mut ex) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, pow(num, ex));
}
unsafe extern "C" fn stdMathSqrt(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, sqrt(num));
}
static mut _pk_doc_stdMathSqrt: *const libc::c_char = b"math.sqrt(value:Number) -> Number\n\nReturns the square root of the value\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc_stdMathAbs: *const libc::c_char = b"math.abs(value:Number) -> Number\n\nReturns the absolute value.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathAbs(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    if num < 0 as libc::c_int as libc::c_double {
        num = -num;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, num);
}
static mut _pk_doc_stdMathSign: *const libc::c_char = b"math.sign(value:Number) -> Number\n\nreturn the sign of the which is one of (+1, 0, -1).\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathSign(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    if num < 0 as libc::c_int as libc::c_double {
        num = -(1 as libc::c_int) as libc::c_double;
    } else if num > 0 as libc::c_int as libc::c_double {
        num = 1 as libc::c_int as libc::c_double;
    } else {
        num = 0 as libc::c_int as libc::c_double;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, num);
}
static mut _pk_doc_stdMathSine: *const libc::c_char = b"math.sin(rad:Number) -> Number\n\nReturn the sine value of the argument [rad] which is an angle expressed in radians.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathSine(mut vm: *mut PKVM) {
    let mut rad: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut rad) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, sin(rad));
}
static mut _pk_doc_stdMathCosine: *const libc::c_char = b"math.cos(rad:Number) -> Number\n\nReturn the cosine value of the argument [rad] which is an angle expressed in radians.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathCosine(mut vm: *mut PKVM) {
    let mut rad: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut rad) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, cos(rad));
}
static mut _pk_doc_stdMathTangent: *const libc::c_char = b"math.tan(rad:Number) -> Number\n\nReturn the tangent value of the argument [rad] which is an angle expressed in radians.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathTangent(mut vm: *mut PKVM) {
    let mut rad: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut rad) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, tan(rad));
}
static mut _pk_doc_stdMathSinh: *const libc::c_char = b"math.sinh(val:Number) -> Number\n\nReturn the hyperbolic sine value of the argument [val].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathSinh(mut vm: *mut PKVM) {
    let mut val: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut val) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, sinh(val));
}
static mut _pk_doc_stdMathCosh: *const libc::c_char = b"math.cosh(val:Number) -> Number\n\nReturn the hyperbolic cosine value of the argument [val].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathCosh(mut vm: *mut PKVM) {
    let mut val: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut val) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, cosh(val));
}
static mut _pk_doc_stdMathTanh: *const libc::c_char = b"math.tanh(val:Number) -> Number\n\nReturn the hyperbolic tangent value of the argument [val].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathTanh(mut vm: *mut PKVM) {
    let mut val: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut val) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, tanh(val));
}
static mut _pk_doc_stdMathArcSine: *const libc::c_char = b"math.asin(num:Number) -> Number\n\nReturn the arcsine value of the argument [num] which is an angle expressed in radians.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathArcSine(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    if num < -(1 as libc::c_int) as libc::c_double
        || (1 as libc::c_int as libc::c_double) < num
    {
        pkSetRuntimeError(
            vm,
            b"Argument should be between -1 and +1\0" as *const u8 as *const libc::c_char,
        );
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, asin(num));
}
static mut _pk_doc_stdMathArcCosine: *const libc::c_char = b"math.acos(num:Number) -> Number\n\nReturn the arc cosine value of the argument [num] which is an angle expressed in radians.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathArcCosine(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    if num < -(1 as libc::c_int) as libc::c_double
        || (1 as libc::c_int as libc::c_double) < num
    {
        pkSetRuntimeError(
            vm,
            b"Argument should be between -1 and +1\0" as *const u8 as *const libc::c_char,
        );
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, acos(num));
}
static mut _pk_doc_stdMathArcTangent: *const libc::c_char = b"math.atan(num:Number) -> Number\n\nReturn the arc tangent value of the argument [num] which is an angle expressed in radians.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathArcTangent(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, atan(num));
}
static mut _pk_doc_stdMathArcTan2: *const libc::c_char = b"math.atan2(y:Number, x:Number) -> Number\n\nThese functions calculate the principal value of the arc tangent of y / x, using the signs of the two arguments to determine the quadrant of the result\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathArcTan2(mut vm: *mut PKVM) {
    let mut y: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut y) {
        return;
    }
    if !pkValidateSlotNumber(vm, 2 as libc::c_int, &mut x) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, atan2(y, x));
}
static mut _pk_doc_stdMathLog10: *const libc::c_char = b"math.log10(value:Number) -> Number\n\nReturn the logarithm to base 10 of argument [value]\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathLog10(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, log10(num));
}
static mut _pk_doc_stdMathRound: *const libc::c_char = b"math.round(value:Number) -> Number\n\nRound to nearest integer, away from zero and return the number.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathRound(mut vm: *mut PKVM) {
    let mut num: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut num) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, round(num));
}
static mut _pk_doc_stdMathRand: *const libc::c_char = b"math.rand() -> Number\n\nReturn a random runber in the range of 0..0x7fff.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdMathRand(mut vm: *mut PKVM) {
    pkSetSlotNumber(
        vm,
        0 as libc::c_int,
        (rand() % 0x7fff as libc::c_int) as libc::c_double,
    );
}
pub unsafe extern "C" fn registerModuleMath(mut vm: *mut PKVM) {
    let mut math: *mut PkHandle = pkNewModule(
        vm,
        b"math\0" as *const u8 as *const libc::c_char,
    );
    pkReserveSlots(vm, 2 as libc::c_int);
    pkSetSlotHandle(vm, 0 as libc::c_int, math);
    pkSetSlotNumber(vm, 1 as libc::c_int, 3.14159265358979323846f64);
    pkSetAttribute(
        vm,
        0 as libc::c_int,
        b"PI\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"floor\0" as *const u8 as *const libc::c_char,
        Some(stdMathFloor as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathFloor,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"ceil\0" as *const u8 as *const libc::c_char,
        Some(stdMathCeil as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathCeil,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"pow\0" as *const u8 as *const libc::c_char,
        Some(stdMathPow as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc_stdMathPow,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"sqrt\0" as *const u8 as *const libc::c_char,
        Some(stdMathSqrt as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathSqrt,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"abs\0" as *const u8 as *const libc::c_char,
        Some(stdMathAbs as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathAbs,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"sign\0" as *const u8 as *const libc::c_char,
        Some(stdMathSign as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathSign,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"sin\0" as *const u8 as *const libc::c_char,
        Some(stdMathSine as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathSine,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"cos\0" as *const u8 as *const libc::c_char,
        Some(stdMathCosine as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathCosine,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"tan\0" as *const u8 as *const libc::c_char,
        Some(stdMathTangent as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathTangent,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"sinh\0" as *const u8 as *const libc::c_char,
        Some(stdMathSinh as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathSinh,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"cosh\0" as *const u8 as *const libc::c_char,
        Some(stdMathCosh as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathCosh,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"tanh\0" as *const u8 as *const libc::c_char,
        Some(stdMathTanh as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathTanh,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"asin\0" as *const u8 as *const libc::c_char,
        Some(stdMathArcSine as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathArcSine,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"acos\0" as *const u8 as *const libc::c_char,
        Some(stdMathArcCosine as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathArcCosine,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"atan\0" as *const u8 as *const libc::c_char,
        Some(stdMathArcTangent as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathArcTangent,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"atan2\0" as *const u8 as *const libc::c_char,
        Some(stdMathArcTan2 as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc_stdMathArcTan2,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"log10\0" as *const u8 as *const libc::c_char,
        Some(stdMathLog10 as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathLog10,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"round\0" as *const u8 as *const libc::c_char,
        Some(stdMathRound as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdMathRound,
    );
    pkModuleAddFunction(
        vm,
        math,
        b"rand\0" as *const u8 as *const libc::c_char,
        Some(stdMathRand as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc_stdMathRand,
    );
    pkRegisterModule(vm, math);
    pkReleaseHandle(vm, math);
}
