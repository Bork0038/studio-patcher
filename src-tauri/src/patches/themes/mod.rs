mod lib;
use lib::*;

use super::{ Patch, PatchType, ReplacementPatch, OffsetPatch};
use crate::binary::{ IDAPat, Binary, Section };
use lazy_static::lazy_static;
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;
use std::fs::read_dir;
use object::LittleEndian;
use iced_x86::{ Decoder, DecoderOptions, Instruction, Code, OpKind,  Encoder, FastFormatter, Register };

pub struct ThemesPatch;

lazy_static! {
    static ref PATCHES: Vec<PatchType> = vec![
        
    ];

    static ref THEME_LOAD_PAT: &'static str = "41 8D 57 2B 48 8D 0D 91 40 05 03 FF 15 ?? ?? ?? ?? 48 89 85 00 01 00 00 41 8D 57 2A 48 8D 0D A9 40 05 03 FF 15 ?? ?? ?? ?? 48 89 85 08 01 00 00 41 8D 57 02 48 8D 8D 70 01 00 00 E8 ?? ?? ?? ?? 4C 8B F0 48 89 85 70 01 00 00 4C 8D 78 10 4C 89 BD 80 01 00 00 48 8B F8 48 8D 9D 00 01 00 00 0F 1F 44 00 00 48 8B D3 48 8B CF FF 15 ?? ?? ?? ?? 48 83 C7 08 48 83 C3 08 48 8D 85 10 01 00 00 48 3B D8 75 E0 48 89 BD A0 01 00 00 48 89 BD 78 01 00 00 4C 8B 0D FB 13 52 02 BA 08 00 00 00 44 8D 42 FA 48 8D 8D 00 01 00 00 E8 ?? ?? ?? ??";
    static ref THEME_LOAD_LEN: usize = THEME_LOAD_PAT.split_whitespace().count();
}

impl ThemesPatch {

    pub fn new() -> Patch {
        Patch {
            name: "themes".into(),
            patch: ThemesPatch::patch
        }
    }

    pub fn patch( binary: Rc<RefCell<Binary>> ) -> Result<(), Box<dyn Error>> {
        let mut themes = vec![
            String::from( ":/Platform/Base/QtUI/themes/DarkTheme.json" ),
            String::from( ":/Platform/Base/QtUI/themes/LightTheme.json" )
        ];

        for file in read_dir("../themes")? {
            let file = file?;
            let metadata = file.metadata()?;

            if metadata.is_file() {
                if let Some(path) = file.path().canonicalize()?.to_str() {
                    themes.push( 
                        String::from( path )
                    );
                }
            }
        }

        let mut section = Vec::new();
        let mut offset_map = Vec::new();
        unsafe {
            for theme in themes {
                offset_map.push( section.len() as u64 );
                section.append(
                    &mut QArrayData::serialize( &theme )
                );
            }
        }

        let section_size = 
            section.len() 
            + offset_map.len() * 8 
            + 8;

        let new_section = Section::new(
            ".themes",
            std::iter::repeat( 0x00 ).take( section_size ).collect::<Vec<u8>>() 
        );

        let mut bin = binary.borrow_mut();
        bin.add_section( new_section );
        bin.reload()?;

        let themes_section = bin
            .get_section_by_name( ".themes" )
            .map_or(
                Err("Failed to find .themes section"), 
                | section | Ok(section)
            )?;

        let text_section = bin
            .get_section_by_name( ".text" )
            .map_or(
                Err("Failed to find .text section"), 
                | section | Ok(section)
            )?;   

        let theme_load_addr = bin
            .scan(
                &IDAPat::new( THEME_LOAD_PAT.clone() ),
                Some( ".text" )
            )
            .map_or(
                Err("Failed to find RBXQT::Theme::Theme"),
                | addr | Ok( addr )
            )?;
            
        let themes_rva = themes_section.header.virtual_address.get(LittleEndian);
        let text_rva = text_section.header.virtual_address.get(LittleEndian);

        let rip = text_rva as u64 + theme_load_addr as u64;
        let mut text_data = text_section.data;

        let mut instructions = {
            let mut instructions = Vec::new();

            let mut decoder = Decoder::with_ip( 
                64, 
                &text_data[ theme_load_addr..theme_load_addr + THEME_LOAD_LEN.clone() ], 
                rip, 
                DecoderOptions::NONE
            );
            
            while decoder.can_decode() {
                let mut instruction = Instruction::new();
                decoder.decode_out( &mut instruction );
    
                instructions.push( instruction );
            }

            instructions
        };

        // find registers needed
        let start_register = {
            let inst = instructions
                .get( 11 )
                .map_or(
                    Err("Failed to find instruction"),
                    | inst | Ok( inst )
                )?;

            if inst.op0_kind() != OpKind::Register {
                Err("Instruction op0 not a register" )
            } else {
                Ok( inst.op0_register() )
            }
        }?;
        
        let end_register = {
            let inst = instructions
                .get( 15 )
                .map_or(
                    Err("Failed to find instruction"),
                    | inst | Ok( inst )
                )?;

            if inst.op0_kind() != OpKind::Register {
                Err("Instruction op0 not a register" )
            } else {
                Ok( inst.op0_register() )
            }
            
        }?;

        let static_start_register =  {
            let inst = instructions
                .get( 12 )
                .map_or(
                    Err("Failed to find instruction"),
                    | inst | Ok( inst )
                )?;

            if inst.op0_kind() != OpKind::Memory {
                Err("Instruction op0 not memory" )
            } else {
                Ok( inst ) 
            }
        }?;

        let offset = themes_rva - ( text_rva + theme_load_addr as u32 );
        let mut encoder = Encoder::new( 64 );
        let mut out_inst = Vec::new();

        // mov rax, [rip]
        // add rax, ????
        {
            let mut inst = Instruction::new();
            let data = [ 0x48, 0x8B, 0x05, 0x00, 0x00, 0x00, 0x00 ];

            let mut decoder = Decoder::with_ip( 64, &data, text_rva as u64 + theme_load_addr as u64, DecoderOptions::NONE );
            decoder.decode_out( &mut inst );
            out_inst.push( inst );

            let mut inst = Instruction::new();
            inst.set_code( Code::Add_rm64_imm32 );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( Register::RAX );
            inst.set_op1_kind( OpKind::Immediate32to64 );
            inst.set_immediate32to64( offset as i64 - data.len() as i64 );

            out_inst.push( inst );
        }

        // mov start, rax
        {
            let mut inst = Instruction::new();

            inst.set_code( Code::Mov_rm64_r64 );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( start_register );
            inst.set_op1_kind( OpKind::Register );
            inst.set_op1_register( Register::RAX );

            out_inst.push( inst );
        }

        // mov [static], rax
        {
            let mut inst = Instruction::new();

            inst.set_code( Code::Mov_rm64_r64 );
            inst.set_op0_kind( OpKind::Memory );
            inst.set_memory_base( static_start_register.memory_base() );
            inst.set_memory_displ_size( static_start_register.memory_displ_size() );
            inst.set_memory_displacement64( static_start_register.memory_displacement64() );
            inst.set_memory_index( static_start_register.memory_index() );
            inst.set_memory_index_scale( static_start_register.memory_index_scale() );
            inst.set_op1_kind( OpKind::Register );
            inst.set_op1_register( Register::RAX );

            out_inst.push( inst );
        }

        // mov end, rax
        {
            let mut inst = Instruction::new();

            inst.set_code( Code::Mov_rm64_r64 );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( end_register );
            inst.set_op1_kind( OpKind::Register );
            inst.set_op1_register( Register::RAX );

            out_inst.push( inst );
        }

        // add end, offset
        {
            let mut inst = Instruction::new();

            inst.set_code( Code::Add_rm64_imm32 );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( end_register );
            inst.set_op1_kind( OpKind::Immediate32to64 );
            inst.set_immediate32to64( offset_map.len() as i64 * 8 );

            out_inst.push( inst );
        }

        let mut encoder = Encoder::new( 64 );
        for inst in out_inst {
            encoder.encode( &inst, theme_load_addr as u64)?;
        }
        
        let mut inst_buf = encoder.take_buffer();
        let mut data_buf = Vec::from_iter( std::iter::repeat( 0x90 ).take( THEME_LOAD_LEN.clone() - inst_buf.len() ) );
        inst_buf.append( &mut data_buf );
        
        for i in 0..inst_buf.len() {
            text_data[ theme_load_addr + i ] = inst_buf[ i ];
        }


        // shift the theme load loop back by ?? bytes to add space for more instructions
        const PADDING_BYTES: usize = 20;

        let loop_start = theme_load_addr + THEME_LOAD_LEN.clone();
        let mut loop_end = loop_start;
        while &text_data[ loop_end..loop_end + 4 ] != &[ 0x48, 0x8B, 0xD0, 0x48 ] {
            loop_end += 1;
        }

        let loop_size = loop_end - loop_start + 3;
        for i in 0..loop_size {
            let i = loop_size - i - 1;

            text_data[ loop_start - PADDING_BYTES - loop_size - 1 + i ] = text_data[ loop_start + i ];
            text_data[ loop_start + i ] = 0x90;
        }
        
        let new_offset = text_data[ loop_start + loop_size - PADDING_BYTES ];

        bin.set_section_data( ".text", text_data )?;
        // let offset = themes_rva - ( text_rva + theme_load_addr as u32 );
        // let theme_array_offset = offset + 3 + section.len() as u32;


        // load theme array start and end sections
        // let mut patch = vec![
        //     0x48, 0x8b, 0x05, 0x00, 0x00, 0x00, 0x00, // mov rax, [rip]
        //     0x48, 0x05, // add rax, ????
        //     0x49, 0x89, 0xc6  // mov r14, rax
        // ];
        // patch.append( &mut u32::to_le_bytes( theme_array_offset ).to_vec() );

        // patch.append( &mut vec![ 
        //     0x48, 0x89, 0xC7, // mov rdi, rax
        //     0x48, 0x81, 0xC7 // add rdi ????
        // ]);
        // patch.append( &mut u32::to_le_bytes( (offset_map.len() * 8) as u32 ).to_vec() );

        // let mut index = 0;
        // for i in 0..patch.len() { 
        //     text_data[ theme_load_addr + index ] = patch[i];
        //     index += 1;
        // };


        Ok(())
    }

}