use libdrm_amdgpu_sys::AMDGPU::DeviceHandle;
use crate::Opt;
mod utils;
use utils::*;

const PANEL_WIDTH: usize = 70;

pub const GFX10_GRBM_INDEX: &[(&str, usize)] = &[
    ("Graphics Pipe", 31),
    ("Texture Pipe", 14),
    // ("Command Processor", 29),
    // ("Global Data Share", 15),
    ("Shader Export", 20),
    ("Shader Processor Interpolator", 22),
    ("Primitive Assembly", 25),
    ("Depth Block", 26),
    ("Color Block", 30),
    ("Geometry Engine", 21),
];

pub const GRBM_INDEX: &[(&str, usize)] = &[
    ("Graphics Pipe", 31),
    ("Texture Pipe", 14),
    // ("Command Processor", 29),
    // ("Global Data Share", 15),
    ("Shader Export", 20),
    ("Shader Processor Interpolator", 22),
    ("Primitive Assembly", 25),
    ("Depth Block", 26),
    ("Color Block", 30),
    ("Vertext Grouper / Tessellator", 17),
    ("Input Assembly", 19),
    ("Work Distributor", 21),
];

pub const GRBM2_INDEX: &[(&str, usize)] = &[
    ("Texture Cache", 25),
    ("Command Processor -  Fetcher", 28),
    ("Command Processor -  Compute", 29),
    ("Command Processor - Graphics", 30),
];

/*
pub const SRBM_INDEX: &[(&str, usize)] = &[
    ("UVD", 19),
];

pub const SRBM2_INDEX: &[(&str, usize)] = &[
    ("VCE0", 7),
//    ("VCE1", 14),
    ("SDMA0", 5),
    ("SDMA1", 6),
//    ("SDMA2", 10),
//    ("SDMA3", 11),
];
*/

pub const CP_STAT_INDEX: &[(&str, usize)] = &[
    ("ROQ_RING_BUSY", 9),
    ("ROQ_INDIRECT1_BUSY", 10),
    ("ROQ_INDIRECT2_BUSY", 11),
    ("ROQ_STATE_BUSY", 12),
    ("DC_BUSY", 13),
    ("UTCL2IU_BUSY", 14),
    ("Prefetch Parser", 15),
    ("MEQ_BUSY", 16),
    ("Micro Engine", 17),
    ("QUERY_BUSY", 18),
    ("SEMAPHORE_BUSY", 19),
    ("INTERRUPT_BUSY", 20),
    ("Surface Sync", 21),
    ("DMA", 22),
    ("RCIU_BUSY", 23),
    ("Scratch Memory", 24),
    ("CE_BUSY", 26),
    ("TCIU_BUSY", 27),
    ("ROQ_CE_RING_BUSY", 28),
    ("ROQ_CE_INDIRECT1_BUSY", 29),
    ("ROQ_CE_INDIRECT2_BUSY", 30),
    ("CP_BUSY", 31),
];

mod pc_type;
pub use pc_type::*;

mod perf_counter;
pub use perf_counter::*;

mod fdinfo;
pub use fdinfo::*;

mod vram_usage;
pub use vram_usage::*;

mod sensors;
pub use sensors::*;

mod gpu_metrics;
pub use gpu_metrics::*;

mod pcie_bw;
pub use pcie_bw::*;
