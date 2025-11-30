// This is responsible for figuring out which version of the exe it is,
// thus where all the tables are.
// Data from: https://moddingwiki.shikadi.net/wiki/Commander_Keen_4-6

use anyhow::{Result, bail};

pub fn determine(exe_size: usize) -> Result<ExeOffsets> {
    println!("Determining version...");
    let version = determine_exe_version(exe_size)?;
    println!("Detected version: {:?}", version);
    Ok(version.offsets())
}

#[derive(Debug)]
enum ExeVersion {
    Keen4_1_0Demo,
    Keen4_1_0,
    Keen4_1_1,
    Keen4_1_2,
    Keen4_1_4,
    Keen4_1_4g,
    Keen5_1_0,
    Keen5_1_4,
    Keen5_1_4g,
    Keen6_1_0Demo,
    Keen6_1_0Promo,
    Keen6_1_0,
    Keen6_1_4,
    Keen6_1_5,
}

fn determine_exe_version(exe_size: usize) -> Result<ExeVersion> {
    // Values from:
    // https://moddingwiki.shikadi.net/wiki/Commander_Keen_4-6
    match exe_size {
        262240 => Ok(ExeVersion::Keen4_1_0Demo),
        258064 => Ok(ExeVersion::Keen4_1_0),
        259232 => Ok(ExeVersion::Keen4_1_1),
        259920 => Ok(ExeVersion::Keen4_1_2),
        263488 => Ok(ExeVersion::Keen4_1_4),
        264864 => Ok(ExeVersion::Keen4_1_4g),
        262176 => Ok(ExeVersion::Keen5_1_0),
        266096 => Ok(ExeVersion::Keen5_1_4),
        267616 => Ok(ExeVersion::Keen5_1_4g),
        236112 => Ok(ExeVersion::Keen6_1_0Demo),
        238368 => Ok(ExeVersion::Keen6_1_0Promo),
        266032 => Ok(ExeVersion::Keen6_1_0),
        271696 => Ok(ExeVersion::Keen6_1_4),
        270896 => Ok(ExeVersion::Keen6_1_5),
        _ => bail!("Unknown exe size: {}", exe_size),
    }    
}

impl ExeVersion {
    fn offsets(&self) -> ExeOffsets {
        match self {
            ExeVersion::Keen4_1_0Demo => ExeOffsets { map_head_offset: 136336, map_head_len: 402 + 23004, graph_head_offset: 159744, graph_head_len: 18780, graph_dict_offset: 229382, graph_dict_len: 1024 },
            ExeVersion::Keen4_1_0 => ExeOffsets { map_head_offset: 156592, map_head_len: 402 + 23004, graph_head_offset: 142352, graph_head_len: 14232, graph_dict_offset: 225782, graph_dict_len: 1024 },
            ExeVersion::Keen4_1_1 => ExeOffsets { map_head_offset: 157568, map_head_len: 402 + 23004, graph_head_offset: 143328, graph_head_len: 14232, graph_dict_offset: 226946, graph_dict_len: 1024 },
            ExeVersion::Keen4_1_2 => ExeOffsets { map_head_offset: 158176, map_head_len: 402 + 23004, graph_head_offset: 143920, graph_head_len: 14256, graph_dict_offset: 227636, graph_dict_len: 1024 },
            ExeVersion::Keen4_1_4 => ExeOffsets { map_head_offset: 161328, map_head_len: 402 + 23004, graph_head_offset: 147072, graph_head_len: 14256, graph_dict_offset: 231158, graph_dict_len: 1024 },
            ExeVersion::Keen4_1_4g => ExeOffsets { map_head_offset: 162576, map_head_len: 402 + 23004, graph_head_offset: 148320, graph_head_len: 14256, graph_dict_offset: 232406, graph_dict_len: 1024 },
            ExeVersion::Keen5_1_0 => ExeOffsets { map_head_offset: 161664, map_head_len: 402 + 23688, graph_head_offset: 146864, graph_head_len: 14796, graph_dict_offset: 229258, graph_dict_len: 1024 },
            ExeVersion::Keen5_1_4 => ExeOffsets { map_head_offset: 165264, map_head_len: 402 + 23688, graph_head_offset: 150464, graph_head_len: 14796, graph_dict_offset: 233156, graph_dict_len: 1024 },
            ExeVersion::Keen5_1_4g => ExeOffsets { map_head_offset: 166640, map_head_len: 402 + 23688, graph_head_offset: 151840, graph_head_len: 14796, graph_dict_offset: 234532, graph_dict_len: 1024 },
            ExeVersion::Keen6_1_0Demo => ExeOffsets { map_head_offset: 137568, map_head_len: 402 + 19152, graph_head_offset: 124464, graph_head_len: 13098, graph_dict_offset: 204352, graph_dict_len: 1024 },
            ExeVersion::Keen6_1_0Promo => ExeOffsets { map_head_offset: 139920, map_head_len: 402 + 19152, graph_head_offset: 126816, graph_head_len: 13098, graph_dict_offset: 206614, graph_dict_len: 1024 },
            ExeVersion::Keen6_1_0 => ExeOffsets { map_head_offset: 157776, map_head_len: 402 + 23904, graph_head_offset: 141088, graph_head_len: 16683, graph_dict_offset: 231698, graph_dict_len: 1024 },
            ExeVersion::Keen6_1_4 => ExeOffsets { map_head_offset: 162944, map_head_len: 402 + 23904, graph_head_offset: 146256, graph_head_len: 16683, graph_dict_offset: 237294, graph_dict_len: 1024 },
            ExeVersion::Keen6_1_5 => ExeOffsets { map_head_offset: 181984, map_head_len: 402 + 23904, graph_head_offset: 165296, graph_head_len: 16683, graph_dict_offset: 236366, graph_dict_len: 1024 },
        }
    }
}

pub struct ExeOffsets {
    pub map_head_offset: usize,
    pub map_head_len: usize,
    pub graph_head_offset: usize,
    pub graph_head_len: usize,
    pub graph_dict_offset: usize,
    pub graph_dict_len: usize,
}
