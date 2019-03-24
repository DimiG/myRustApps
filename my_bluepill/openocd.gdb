# GDB script for loading and running programs in STM32 Blue Pill

# Send GDB commands to OpenOCD, which listens on port 3333.  Extend the timeout.
set remotetimeout 100000
target extended-remote :3333

# Print demangled symbols by default
set print asm-demangle on

# Enable ARM semihosting to show debug console output in OpenOCD console
monitor arm semihosting enable

# Specify the target program to be debugged
# Must be specified here (not the command line)
#file target/thumbv7m-none-eabi/debug/my_bluepill

# Load the program into device memory
#load

# Run the program
#continue

##########################################################################
# Optional Commands

# Set backtrace limit to not have infinite backtrace loops
#set backtrace limit 32

# Detect unhandled exceptions, hard faults and panics
#break DefaultHandler
#break HardFault
#break rust_begin_unwind

# # Send captured ITM to the file itm.fifo
# # (the microcontroller SWO pin must be connected to the programmer SWO pin)
# # 8000000 must match the core clock frequency
# monitor tpiu config internal itm.txt uart off 8000000

# # OR: make the microcontroller SWO pin output compatible with UART (8N1)
# # 8000000 must match the core clock frequency
# # 2000000 is the frequency of the SWO pin
# monitor tpiu config external uart off 8000000 2000000

# # Enable ITM port 0
# monitor itm port 0 on

