//! Abstracts over registers in a [crate::core::Core]
//!
//! Registers are grouped in multiple groups within a core.

use std::{ffi::CStr, fmt::Debug, io::Cursor};

use anyhow::Context;
use byteorder::ReadBytesExt;

use crate::{
    error::expect_error,
    mcd_bindings::{mcd_register_group_st, mcd_register_info_st},
    MCD_LIB,
};

use super::core::Core;

/// Specifies all register groups in the core
pub struct RegisterGroups<'a> {
    core: &'a Core<'a>,
    register_groups: Vec<mcd_register_group_st>,
}

impl<'a> RegisterGroups<'a> {
    pub(crate) fn from_core(core: &'a Core<'a>) -> anyhow::Result<Self> {
        let mut number_of_groups = 0;

        let result = unsafe {
            MCD_LIB.mcd_qry_reg_groups_f(core.core, 0, &mut number_of_groups, core::ptr::null_mut())
        };

        if result != 0 {
            return Err(expect_error(Some(core)))
                .with_context(|| "Library reported an internal error");
        }

        let mut register_groups = vec![mcd_register_group_st::default(); number_of_groups as usize];

        let result = unsafe {
            MCD_LIB.mcd_qry_reg_groups_f(
                core.core,
                0,
                &mut number_of_groups,
                register_groups.as_mut_ptr(),
            )
        };

        if result != 0 {
            return Err(expect_error(Some(core)))
                .with_context(|| "Unable to query register groups");
        }

        Ok(Self {
            core,
            register_groups,
        })
    }

    /// Get a group from this list of groups
    ///
    /// TODO Add/Reference documentation what the signifcance of index is. For
    /// a trivial setup with the Aurix Lite Kit v2 connected over MicroUSB, this
    /// must be 0.
    pub fn get_group(&self, index: usize) -> anyhow::Result<RegisterGroup> {
        let register_group = &self.register_groups[index];

        let mut number_of_registers = Default::default();

        let result = unsafe {
            MCD_LIB.mcd_qry_reg_map_f(
                self.core.core,
                register_group.reg_group_id,
                0,
                &mut number_of_registers,
                core::ptr::null_mut(),
            )
        };
        if result != 0 {
            return Err(expect_error(Some(self.core)))
                .with_context(|| "Unable to query the number of registers in register group");
        }

        // This should be the same, isn't it?
        assert_eq!(number_of_registers, register_group.n_registers);

        let mut registers = vec![mcd_register_info_st::default(); number_of_registers as usize];

        let result = unsafe {
            MCD_LIB.mcd_qry_reg_map_f(
                self.core.core,
                register_group.reg_group_id,
                0,
                &mut number_of_registers,
                registers.as_mut_ptr(),
            )
        };

        if result != 0 {
            return Err(expect_error(Some(self.core)))
                .with_context(|| "Unable to query registers in group");
        }

        Ok(RegisterGroup {
            core: self.core,
            registers,
        })
    }
}

#[derive(Debug)]
pub struct RegisterGroup<'a> {
    core: &'a Core<'a>,
    registers: Vec<mcd_register_info_st>,
}

impl<'a> RegisterGroup<'a> {
    /// Iterate over all registers in this group
    pub fn registers(&'a self) -> RegisterIterator<'a> {
        RegisterIterator {
            core: self.core,
            iter: self.registers.iter(),
        }
    }

    /// Obtain the register with the given name from this group
    pub fn register(&'a self, name: &str) -> Option<Register<'a>> {
        self.registers().find(|r| r.name() == name)
    }
}

/// Allows to iterate over registers in a [RegisterGroup]
pub struct RegisterIterator<'a> {
    core: &'a Core<'a>,
    iter: std::slice::Iter<'a, mcd_register_info_st>,
}

impl<'a> Iterator for RegisterIterator<'a> {
    type Item = Register<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|register| Register {
            core: self.core,
            register,
        })
    }
}

/// Represents a register within a [RegisterGroup]
pub struct Register<'a> {
    core: &'a Core<'a>,
    register: &'a mcd_register_info_st,
}

impl<'a> Register<'a> {
    /// Read the value of this register
    pub fn read(&self) -> anyhow::Result<u32> {
        let data = self.core.read_bytes(self.register.addr.address, 4)?;
        let mut cursor = Cursor::new(data);
        cursor
            .read_u32::<byteorder::LE>()
            .with_context(|| "could not transform data to register value")
    }

    /// The name of the register as reported from the debug controller
    pub fn name(&self) -> String {
        unsafe { CStr::from_ptr(&self.register.regname[0] as *const i8) }
            .to_str()
            .unwrap()
            .to_owned()
    }
}

impl<'a> Debug for Register<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self
            .read()
            .map(|value| format!("{value:#8X}"))
            .unwrap_or_else(|_| "<read error>".to_owned());

        f.debug_struct("Register")
            .field("name", &self.name())
            .field("value", &value)
            .finish()
    }
}
