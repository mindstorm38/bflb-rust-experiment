//! PWM (Pulse-Width Modulation) related clocks.

use crate::arch::bl808::GLB;


pub unsafe fn set_pwm0_enable(enable: bool) {
    GLB.cgen_cfg1().modify(|reg| reg.cgen_s1a_pwm().set(enable as _));
}
