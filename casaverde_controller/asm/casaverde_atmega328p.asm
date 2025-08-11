; ATmega328P Assembly Program
; Target: 
; Assembler: AVR Assembler

.nolist
.include "m328def.inc" ; include definitions
.list

; Define constants
.equ DATA_BYTE = 0x55
.equ RAM_ADDR = 0x0100 ; RAM starts at 0x0100

; Reset 
.org 0x0000
RESET:
  ; Stack pointer

  ; Port B, Pin 0

  ; Store a byte of data in RAM

  ; Power on indicator

  ; Loop

  ; Subroutine
