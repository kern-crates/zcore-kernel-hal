#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use trapframe::TrapFrame;

// Reference: https://wiki.osdev.org/Exceptions
const DivideError: u8 = 0;
const Debug: u8 = 1;
const NonMaskableInterrupt: u8 = 2;
const Breakpoint: u8 = 3;
const Overflow: u8 = 4;
const BoundRangeExceeded: u8 = 5;
const InvalidOpcode: u8 = 6;
const DeviceNotAvailable: u8 = 7;
const DoubleFault: u8 = 8;
const CoprocessorSegmentOverrun: u8 = 9;
const InvalidTSS: u8 = 10;
const SegmentNotPresent: u8 = 11;
const StackSegmentFault: u8 = 12;
const GeneralProtectionFault: u8 = 13;
const PageFault: u8 = 14;
const FloatingPointException: u8 = 16;
const AlignmentCheck: u8 = 17;
const MachineCheck: u8 = 18;
const SIMDFloatingPointException: u8 = 19;
const VirtualizationException: u8 = 20;
const SecurityException: u8 = 30;

const IRQ0: u8 = 32;

fn breakpoint() {
    panic!("\nEXCEPTION: Breakpoint");
}

fn double_fault(tf: &TrapFrame) {
    panic!("\nEXCEPTION: Double Fault\n{:#x?}", tf);
}

fn page_fault(tf: &mut TrapFrame) {
    panic!(
        "\nEXCEPTION: Page Fault @ {:#x?}\n{:#x?}",
        crate::context::fetch_fault_vaddr(),
        tf
    );
}

#[no_mangle]
pub extern "C" fn trap_handler(tf: &mut TrapFrame) {
    trace!("Interrupt: {:#x} @ CPU{}", tf.trap_num, 0); // TODO 0 should replace in multi-core case
    match tf.trap_num as u8 {
        Breakpoint => breakpoint(),
        DoubleFault => double_fault(tf),
        PageFault => page_fault(tf),
        IRQ0..=63 => crate::interrupt::handle_irq(tf.trap_num as u32),
        _ => panic!("Unhandled interrupt {:x} {:#x?}", tf.trap_num, tf),
    }
}