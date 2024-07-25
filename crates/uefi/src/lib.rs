#![no_std]

///
/// The UEFI specification v2.10
///
/// https://uefi.org/specs/UEFI/2.10/
///

///
/// https://uefi.org/specs/UEFI/2.10/Apx_D_Status_Codes.html?highlight=efi_status#efi-status-success-codes-high-bit-clear-apx-d-status-codes
///
#[repr(C)]
pub enum EFIStatus {
    ///
    /// The operation completed successfully.
    ///
    Success = 0
}

///
/// https://uefi.org/specs/UEFI/2.10/02_Overview.html?highlight=uintn#common-uefi-data-types
///
pub type UIntN = usize;

pub type EFIGUID = u32;

///
/// Data structure that precedes all of the standard EFI table types.
///
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#id4
///
#[repr(C)]
pub struct EFITableHeader {
    ///
    /// A 64-bit signature that identifies the type of table that follows.
    /// Unique signatures have been generated for the EFI System Table,
    /// the EFI Boot Services Table, and the EFI Runtime Services Table.
    ///
    pub signature: u64,

    ///
    /// The revision of the EFI Specification to which this table conforms.
    /// The upper 16 bits of this field contain the major revision value,
    /// and the lower 16 bits contain the minor revision value. The minor
    /// revision values are binary coded decimals and are limited to the
    /// range of 00..99.
    ///
    /// When printed or displayed UEFI spec revision is referred as (Major revision).(Minor revision upper decimal).(Minor revision lower decimal) or (Major revision).(Minor revision upper decimal) in case Minor revision lower decimal is set to 0. For example:
    ///
    /// Specification with the revision value ((2<<16) | (30)) would be referred as 2.3;
    ///
    /// A specification with the revision value ((2<<16) | (31)) would be referred as 2.3.1
    ///
    pub revision: u32,

    ///
    /// The size, in bytes, of the entire table including the FI_TABLE_HEADER.
    ///
    pub header_size: u32,

    ///
    /// The 32-bit CRC for the entire table. This value is computed by setting this field to 0, and computing the 32-bit CRC for HeaderSize bytes.
    ///
    pub crc32: u32,

    ///
    /// Reserved field that must be set to 0.
    ///
    pub reserved: u32,
}

/// https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html#efi-boot-services-installprotocolinterface
pub type EFIHandle = *mut core::ffi::c_void;

/// https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html#efi-boot-services-createevent
pub type EFIEvent = *mut core::ffi::c_void;

/// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_input_protocol#efi-simple-text-input-protocol
#[repr(C)]
pub struct EFIInputKey {
    pub scan_code: u16,
    pub unicode_char: u16,
}

/// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_input_protocol#efi-simple-text-input-protocol
#[repr(C)]
pub struct EFISimpleTextInputProtocol {
    // https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html#efi-simple-text-input-protocol-reset
    pub reset: extern "efiapi" fn(this: &EFISimpleTextInputProtocol, extended_verification: bool) -> EFIStatus,

    // https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html#efi-simple-text-input-protocol-readkeystroke
    pub read_key_stroke: extern "efiapi" fn(this: &EFISimpleTextInputProtocol, key: *mut EFIInputKey) -> EFIStatus,

    // https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html#efi-boot-services-waitforevent
    pub wait_for_key: extern "efiapi" fn(number_of_events: UIntN, events: *mut EFIEvent, index: *mut UIntN) -> EFIStatus,
}

/// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol
#[repr(C)]
pub struct EFISimpleTextOutputMode {
    ///
    /// The number of modes supported by QueryMode() and SetMode().
    ///
    pub max_mode: i32,

    ///
    /// The text mode of the output device(s).
    ///
    pub mode: i32,

    ///
    /// The current character output attribute.
    ///
    pub attribute: i32,

    ///
    /// The cursor’s column.
    ///
    pub cursor_column: i32,

    ///
    /// The cursor’s row.
    ///
    pub cursor_row: i32,

    ///
    /// The cursor is currently visible or not.
    ///
    pub cursor_visible: bool,
}

///
/// This protocol is used to control text-based output devices.
///
/// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol
///
#[repr(C)]
pub struct EFISimpleTextOutputProtocol {
    ///
    /// Resets the text output device hardware.
    ///
    /// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol-reset
    ///
    pub reset: extern "efiapi" fn(this: &EFISimpleTextOutputProtocol, extended_verification: bool) -> EFIStatus,

    ///
    /// Writes a string to the output device.
    ///
    /// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol-outputstring
    ///
    pub output_string: extern "efiapi" fn(this: &EFISimpleTextOutputProtocol, string: *const u16) -> EFIStatus,

    ///
    /// Verifies that all characters in a string can be output to the target device.
    ///
    /// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol-teststring
    ///
    pub test_string: extern "efiapi" fn(this: &EFISimpleTextOutputProtocol, string: *const u16) -> EFIStatus,

    ///
    /// Returns information for an available text mode that the output device(s) supports.
    ///
    /// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol-querymode
    ///
    pub query_mode: extern "efiapi" fn(this: &EFISimpleTextOutputProtocol, mode_number: UIntN, columns: *mut UIntN, rows: *mut UIntN) -> EFIStatus,

    ///
    /// Sets the output device(s) to a specified mode.
    ///
    /// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol-setmode
    ///
    pub set_mode: extern "efiapi" fn(this: &EFISimpleTextOutputProtocol, mode_number: UIntN) -> EFIStatus,

    ///
    /// Sets the background and foreground colors for the OutputString(), EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString() and ClearScreen() EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.ClearScreen() functions.
    ///
    /// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol-setattribute
    ///
    pub set_attribute: extern "efiapi" fn(this: &EFISimpleTextOutputProtocol, attribute: UIntN) -> EFIStatus,

    ///
    /// Clears the output device(s) display to the currently selected background color.
    ///
    /// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol-clearscreen
    ///
    pub clear_screen: extern "efiapi" fn(this: &EFISimpleTextOutputProtocol) -> EFIStatus,

    ///
    /// Sets the current coordinates of the cursor position.
    ///
    /// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol-setcursorposition
    ///
    pub set_cursor_position: extern "efiapi" fn(this: &EFISimpleTextOutputProtocol, column: UIntN, row: UIntN) -> EFIStatus,

    ///
    /// Makes the cursor visible or invisible.
    ///
    /// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html?highlight=efi_simple_text_output_protocol#efi-simple-text-output-protocol-enablecursor
    ///
    pub enable_cursor: extern "efiapi" fn(this: &EFISimpleTextOutputProtocol, visible: bool) -> EFIStatus,

    ///
    /// Pointer to SIMPLE_TEXT_OUTPUT_MODE data.
    ///
    pub mode: *const EFISimpleTextOutputMode,
}

impl EFISimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EFIStatus {
        unsafe { (self.reset)(self, extended_verification) }
    }

    pub fn output_string(&self, string: *const u16) -> EFIStatus {
        unsafe { (self.output_string)(self, string) }
    }
}

///
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#efi-runtime-services-table
///
#[repr(C)]
pub struct EFIRuntimeServices {}

///
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#efi-boot-services-table
///
#[repr(C)]
pub struct EFIBootServices {}

///
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#efi-configuration-table-properties-table
///
#[repr(C)]
pub struct EFIConfigurationTable {
    pub vendor_guid: EFIGUID,
    pub vendor_table: *const core::ffi::c_void,
}

///
/// Contains pointers to the runtime and boot services tables.
///
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#id6
///
#[repr(C)]
pub struct EFISystemTable {
    ///
    /// The table header for the EFI System Table.
    ///
    pub hdr: EFITableHeader,

    ///
    /// A pointer to a null terminated string that identifies the vendor that produces the system firmware for the platform.
    ///
    pub firmware_vendor: *const u16,

    ///
    /// A firmware vendor specific value that identifies the revision of the system firmware for the platform.
    ///
    pub firmware_revision: u32,

    ///
    /// The handle for the active console input device.
    ///
    pub console_in_handle: EFIHandle,

    ///
    /// A pointer to the EFI_SIMPLE_TEXT_INPUT_PROTOCOL interface that is associated with ConsoleInHandle.
    ///
    pub con_in: *mut EFISimpleTextInputProtocol,

    ///
    /// The handle for the active console output device.
    ///
    pub console_out_handle: EFIHandle,

    ///
    /// A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is associated with ConsoleOutHandle.
    ///
    pub con_out: *mut EFISimpleTextOutputProtocol,

    ///
    /// The handle for the active standard error console device.
    ///
    pub standard_error_handle: EFIHandle,

    ///
    /// A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is associated with StandardErrorHandle.
    ///
    pub std_err: *mut EFISimpleTextOutputProtocol,

    ///
    /// A pointer to the EFI Runtime Services Table.
    ///
    pub runtime_services: *mut EFIRuntimeServices,

    ///
    /// A pointer to the EFI Boot Services Table
    ///
    pub boot_services: *mut EFIBootServices,

    ///
    /// The number of system configuration tables in the buffer ConfigurationTable.
    ///
    pub number_of_table_entries: UIntN,

    ///
    /// A pointer to the system configuration tables. The number of entries in the table is NumberOfTableEntries.
    ///
    pub configuration_table: *mut EFIConfigurationTable,
}

impl EFISystemTable {
    pub fn get_con_out(&self) -> &mut EFISimpleTextOutputProtocol {
        unsafe { &mut *self.con_out }
    }
}