//! exFAT filesystem structures and parsing
//!
//! This module implements the core exFAT filesystem structures including
//! the boot sector, file allocation table, and directory entries.

/// exFAT Boot Sector
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BootSector {
    /// Jump boot code
    pub jump_boot: [u8; 3],
    /// File system name ("EXFAT   ")
    pub fs_name: [u8; 8],
    /// Must be zeros
    pub must_be_zero: [u8; 53],
    /// Partition offset in sectors
    pub partition_offset: u64,
    /// Volume length in sectors
    pub volume_length: u64,
    /// FAT offset in sectors
    pub fat_offset: u32,
    /// FAT length in sectors
    pub fat_length: u32,
    /// Cluster heap offset in sectors
    pub cluster_heap_offset: u32,
    /// Cluster count
    pub cluster_count: u32,
    /// First cluster of root directory
    pub root_dir_cluster: u32,
    /// Volume serial number
    pub volume_serial: u32,
    /// File system revision (major.minor)
    pub fs_revision: u16,
    /// Volume flags
    pub volume_flags: u16,
    /// Bytes per sector (power of 2)
    pub bytes_per_sector_shift: u8,
    /// Sectors per cluster (power of 2)
    pub sectors_per_cluster_shift: u8,
    /// Number of FATs (should be 1 for exFAT)
    pub num_fats: u8,
    /// Drive select
    pub drive_select: u8,
    /// Percent in use
    pub percent_in_use: u8,
    /// Reserved
    pub reserved: [u8; 7],
    /// Boot code
    pub boot_code: [u8; 390],
    /// Boot signature (0xAA55)
    pub boot_signature: u16,
}

impl BootSector {
    /// Size of boot sector in bytes
    pub const SIZE: usize = 512;
    
    /// Verify boot sector signature and filesystem name
    pub fn is_valid(&self) -> bool {
        self.boot_signature == 0xAA55 && 
        &self.fs_name == b"EXFAT   "
    }
    
    /// Get bytes per sector
    pub fn bytes_per_sector(&self) -> u32 {
        1 << self.bytes_per_sector_shift
    }
    
    /// Get sectors per cluster
    pub fn sectors_per_cluster(&self) -> u32 {
        1 << self.sectors_per_cluster_shift
    }
    
    /// Get bytes per cluster
    pub fn bytes_per_cluster(&self) -> u32 {
        self.bytes_per_sector() * self.sectors_per_cluster()
    }
}

/// exFAT Directory Entry Type
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    /// End of directory marker
    EndOfDirectory = 0x00,
    /// Allocation bitmap
    AllocationBitmap = 0x81,
    /// Up-case table
    UpCaseTable = 0x82,
    /// Volume label
    VolumeLabel = 0x83,
    /// File directory entry
    File = 0x85,
    /// Volume GUID
    VolumeGuid = 0xA0,
    /// Stream extension
    StreamExtension = 0xC0,
    /// File name extension
    FileName = 0xC1,
    /// Vendor extension
    VendorExtension = 0xE0,
    /// Unknown type
    Unknown = 0xFF,
}

impl From<u8> for EntryType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => EntryType::EndOfDirectory,
            0x81 => EntryType::AllocationBitmap,
            0x82 => EntryType::UpCaseTable,
            0x83 => EntryType::VolumeLabel,
            0x85 => EntryType::File,
            0xA0 => EntryType::VolumeGuid,
            0xC0 => EntryType::StreamExtension,
            0xC1 => EntryType::FileName,
            0xE0 => EntryType::VendorExtension,
            _ => EntryType::Unknown,
        }
    }
}

/// Generic Directory Entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct DirectoryEntry {
    /// Entry type
    pub entry_type: u8,
    /// Entry data (31 bytes)
    pub data: [u8; 31],
}

impl DirectoryEntry {
    /// Size of directory entry in bytes
    pub const SIZE: usize = 32;
    
    /// Get entry type
    pub fn get_type(&self) -> EntryType {
        EntryType::from(self.entry_type)
    }
    
    /// Check if entry is in use
    pub fn is_in_use(&self) -> bool {
        self.entry_type != 0x00 && (self.entry_type & 0x80) != 0
    }
}

/// File Directory Entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FileEntry {
    /// Entry type (0x85)
    pub entry_type: u8,
    /// Secondary count (number of secondary entries)
    pub secondary_count: u8,
    /// Set checksum
    pub set_checksum: u16,
    /// File attributes
    pub file_attributes: u16,
    /// Reserved
    pub reserved1: u16,
    /// Create timestamp
    pub create_timestamp: u32,
    /// Last modified timestamp
    pub last_modified_timestamp: u32,
    /// Last accessed timestamp
    pub last_accessed_timestamp: u32,
    /// Create 10ms increment
    pub create_10ms: u8,
    /// Last modified 10ms increment
    pub last_modified_10ms: u8,
    /// Create UTC offset
    pub create_utc_offset: u8,
    /// Last modified UTC offset
    pub last_modified_utc_offset: u8,
    /// Last accessed UTC offset
    pub last_accessed_utc_offset: u8,
    /// Reserved
    pub reserved2: [u8; 7],
}

/// File attributes
pub mod file_attributes {
    /// Read-only
    pub const READ_ONLY: u16 = 0x01;
    /// Hidden
    pub const HIDDEN: u16 = 0x02;
    /// System
    pub const SYSTEM: u16 = 0x04;
    /// Directory
    pub const DIRECTORY: u16 = 0x10;
    /// Archive
    pub const ARCHIVE: u16 = 0x20;
}

/// Stream Extension Directory Entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct StreamExtensionEntry {
    /// Entry type (0xC0)
    pub entry_type: u8,
    /// General secondary flags
    pub flags: u8,
    /// Reserved
    pub reserved1: u8,
    /// Name length
    pub name_length: u8,
    /// Name hash
    pub name_hash: u16,
    /// Reserved
    pub reserved2: u16,
    /// Valid data length
    pub valid_data_length: u64,
    /// Reserved
    pub reserved3: u32,
    /// First cluster
    pub first_cluster: u32,
    /// Data length
    pub data_length: u64,
}

/// File Name Directory Entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FileNameEntry {
    /// Entry type (0xC1)
    pub entry_type: u8,
    /// General secondary flags
    pub flags: u8,
    /// File name characters (UTF-16)
    pub file_name: [u16; 15],
}

/// exFAT File Allocation Table Entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FatEntry {
    /// Free cluster
    Free,
    /// Next cluster in chain
    Next(u32),
    /// Bad cluster
    Bad,
    /// End of cluster chain
    EndOfChain,
}

impl FatEntry {
    /// Parse FAT entry from u32 value
    pub fn from_u32(value: u32) -> Self {
        match value {
            0x00000000 => FatEntry::Free,
            0xFFFFFFF7 => FatEntry::Bad,
            0xFFFFFFF8..=0xFFFFFFFF => FatEntry::EndOfChain,
            cluster => FatEntry::Next(cluster),
        }
    }
    
    /// Convert FAT entry to u32 value
    pub fn to_u32(&self) -> u32 {
        match self {
            FatEntry::Free => 0x00000000,
            FatEntry::Next(cluster) => *cluster,
            FatEntry::Bad => 0xFFFFFFF7,
            FatEntry::EndOfChain => 0xFFFFFFFF,
        }
    }
}

#[cfg(all(test, not(target_os = "uefi")))]
mod tests {
    use super::*;
    use core::mem;
    
    #[test]
    fn test_boot_sector_size() {
        assert_eq!(mem::size_of::<BootSector>(), BootSector::SIZE);
    }
    
    #[test]
    fn test_directory_entry_size() {
        assert_eq!(mem::size_of::<DirectoryEntry>(), DirectoryEntry::SIZE);
    }
    
    #[test]
    fn test_fat_entry_parsing() {
        assert_eq!(FatEntry::from_u32(0x00000000), FatEntry::Free);
        assert_eq!(FatEntry::from_u32(0xFFFFFFF7), FatEntry::Bad);
        assert_eq!(FatEntry::from_u32(0xFFFFFFFF), FatEntry::EndOfChain);
        assert_eq!(FatEntry::from_u32(0x12345), FatEntry::Next(0x12345));
    }
}
