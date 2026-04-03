#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

// Print to host through serial interface 
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
            concat!($fmt: "\n"), $($arg)*));
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use uart_16550::SerialPort;
    let mut serial_port = unsafe{SerialPort::new(0x3F8)};

    // let mut serial_port = SerialPort;
    serial_port.write_fmt(args).unwrap();
}
