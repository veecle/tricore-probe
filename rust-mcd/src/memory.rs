//! Abstracts over memory spaces and blocks within
//!
//! TODO The implementations in this module are poorly documented (due to my
//! poor understanding of the concept) and mostly untested.
//!
#![allow(dead_code)]
use std::{ffi::CStr, fmt::Debug, ptr};

use anyhow::Context;

use crate::{
    core::Core,
    error::expect_error,
    mcd_bindings::{mcd_memblock_st, mcd_memspace_st, MCD_MEM_BLOCK_NOPARENT},
    MCD_LIB,
};

pub struct MemorySpace<'a> {
    inner: mcd_memspace_st,
    core: &'a Core<'a>,
}

impl<'a> MemorySpace<'a> {
    pub fn get_all(core: &'a Core) -> Vec<MemorySpace<'a>> {
        let mut query_spaces = 0;
        let result = unsafe {
            MCD_LIB.mcd_qry_mem_spaces_f(core.core.as_ptr(), 0, &mut query_spaces, ptr::null_mut())
        };
        assert_eq!(result, 0);

        println!("Found {:?} spaces", &query_spaces);
        let mut reserved_spaces = Vec::new();
        for _ in 0..query_spaces {
            reserved_spaces.push(mcd_memspace_st::default())
        }

        let result = unsafe {
            MCD_LIB.mcd_qry_mem_spaces_f(
                core.core.as_ptr(),
                0,
                &mut query_spaces,
                reserved_spaces.as_mut_ptr(),
            )
        };
        assert_eq!(result, 0);

        reserved_spaces
            .into_iter()
            .map(|inner| MemorySpace { inner, core })
            .collect()
    }

    pub fn get_blocks(&self) -> anyhow::Result<Vec<MemoryBlock>> {
        log::trace!("Querying for the number of memory spaces");

        let mut result_block_count = 0;

        let result = unsafe {
            MCD_LIB.mcd_qry_mem_blocks_f(
                self.core.core.as_ptr(),
                self.inner.mem_space_id,
                0,
                &mut result_block_count,
                ptr::null_mut(),
            )
        };

        if result != 0 {
            return Err(expect_error(Some(self.core)))
                .with_context(|| "MCD library reported an error");
        }

        log::trace!(
            "Querying additional information for {:?} blocks",
            &result_block_count
        );

        let mut reserved_spaces = vec![mcd_memblock_st::default(); result_block_count as usize];

        let result = unsafe {
            MCD_LIB.mcd_qry_mem_blocks_f(
                self.core.core.as_ptr(),
                self.inner.mem_space_id,
                0,
                &mut result_block_count,
                reserved_spaces.as_mut_ptr(),
            )
        };
        if result != 0 {
            return Err(expect_error(Some(self.core)))
                .with_context(|| "MCD library reported an error");
        }

        Ok(reserved_spaces
            .into_iter()
            .map(|inner| MemoryBlock {
                inner,
                core: self.core,
            })
            .collect())
    }

    /// The name of this space as reported from the debug controller
    pub fn get_name(&self) -> &str {
        unsafe { CStr::from_ptr(&self.inner.mem_space_name[0] as *const i8) }
            .to_str()
            .unwrap()
    }
}

impl<'a> Debug for MemorySpace<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemorySpace")
            .field("space_id", &self.inner.mem_space_id)
            .field("space_name", &self.get_name())
            .field("min_address", &format_args!("{:#X}", self.inner.min_addr))
            .field("max_address", &format_args!("{:#X}", self.inner.max_addr))
            .field("count_inner_blocks", &self.inner.num_mem_blocks)
            .finish()
    }
}

pub struct MemoryBlock<'a> {
    core: &'a Core<'a>,
    inner: mcd_memblock_st,
}

impl<'a> MemoryBlock<'a> {
    pub fn parent(&self) -> Option<u32> {
        if self.inner.parent_id == MCD_MEM_BLOCK_NOPARENT {
            None
        } else {
            Some(self.inner.parent_id)
        }
    }

    /// The name of this block as reported from the debug controller
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(&self.inner.mem_block_name[0] as *const i8) }
            .to_str()
            .unwrap()
    }
}

impl<'a> Debug for MemoryBlock<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryBlock")
            .field("block_id", &self.inner.mem_block_id)
            .field("block_name", &self.name())
            .field("parent", &self.parent())
            .field("start", &self.inner.start_addr)
            .field("end", &self.inner.end_addr)
            .finish()
    }
}
