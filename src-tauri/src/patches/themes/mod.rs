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
    static ref PATCHES: Vec<PatchType> = vec![ ];

    static ref THEME_LOAD_PAT: &'static str = "41 8D 57 2B 48 8D 0D ?? ?? ?? ?? FF 15 ?? ?? ?? ?? 48 89 85 00 01 00 00 41 8D 57 2A 48 8D 0D ?? ?? ?? ?? FF 15 ?? ?? ?? ?? 48 89 85 08 01 00 00 41 8D 57 02 48 8D 8D 70 01 00 00 E8 ?? ?? ?? ?? 4C 8B F0 48 89 85 70 01 00 00 4C 8D 78 10 4C 89 BD 80 01 00 00 48 8B F8 48 8D 9D 00 01 00 00 0F 1F 44 00 00 48 8B D3 48 8B CF FF 15 ?? ?? ?? ?? 48 83 C7 08 48 83 C3 08 48 8D 85 10 01 00 00 48 3B D8 75 E0 48 89 BD A0 01 00 00 48 89 BD 78 01 00 00 4C 8B 0D ?? ?? ?? ?? BA 08 00 00 00 44 8D 42 FA 48 8D 8D 00 01 00 00 E8 ?? ?? ?? ?? 49 8B C6 48 89 45 60 4C 3B F7 0F 84 10 0F 00 00 48 BB B3 01 00 00 00 01 00 00 0F 1F 40 00 0F 1F 84 00 00 00 00 00 48 8B D0";
    static ref THEME_LOAD_LEN: usize = THEME_LOAD_PAT.split_whitespace().count();

    static ref THEME_JNZ_PAT: &'static str = "48 3B C7 0F 85 14 F1 FF FF 4C 8B B5 70 01 00 00 4C 8B BD 80 01 00 00 4D 85 F6";
    static ref THEME_JNZ_LEN: usize = THEME_JNZ_PAT.split_whitespace().count();
}

impl ThemesPatch {

    pub fn new() -> Patch {
        Patch {
            name: "themes".into(),
            patch: ThemesPatch::patch
        }
    }

    pub fn decode_instructions<Data: AsRef<[u8]>>( data: Data, rip: u64 ) -> Vec<Instruction> {
        let mut insts = Vec::new();

        let mut decoder = Decoder::with_ip( 64, data.as_ref(), rip, DecoderOptions::NONE );
        while decoder.can_decode() {
            let mut instruction = Instruction::new();
            decoder.decode_out( &mut instruction );

            insts.push( instruction );
        }

        insts
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

        let mut instructions = ThemesPatch::decode_instructions( &text_data[ theme_load_addr..theme_load_addr + THEME_LOAD_LEN.clone() ], rip );

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

        let offset = themes_rva - ( text_rva + theme_load_addr as u32 ) - 7 ;
        let mut encoder = Encoder::new( 64 );
        let mut out_inst = Vec::new();

        {
            // lea rax, [rip]
            let mut inst = Instruction::new();
            inst.set_code( Code::Lea_r64_m );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( Register::RAX );
            inst.set_op1_kind( OpKind::Memory );
            inst.set_memory_base( Register::RIP );
            inst.set_memory_displacement32( 7 );

            out_inst.push( inst );
            
            // add rax, ????
            let mut inst = Instruction::new();
            inst.set_code( Code::Add_rm64_imm32 );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( Register::RAX );
            inst.set_op1_kind( OpKind::Immediate32to64 );
            inst.set_immediate32to64( offset as i64 + section.len() as i64 );

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

        let insts_clone = [ 
            12, // mov [static], rax
            13, // lea reg, [rax+??]
            14, // mov [static], reg
            26, // mov [static], start
            27, // mov [static], start
            33, // mov rax, register
            34, // mov [static], rax
            35, // cmp reg, reg
            36, // je addy
            37, // mov reg, ????
            38, // nop dword ptr [reg]
            39, // nop dword ptr [reg+reg]
            // 40, // mov reg, reg
        ];

        for i in insts_clone {
            let inst = instructions
                .get( i )
                .map_or(
                    Err("Failed to find instruction"),
                    | inst | Ok( inst )
                )?;

            out_inst.push( *inst );
        }

        let mut loop_insts = Vec::new();
        {
            let instruction = instructions
                .get( 40 )
                .map_or(
                    Err("Failed to find instruction"),
                    | inst | Ok( inst )
                )?;

            let reg_0 = instruction.op0_register();
            let reg_1 = instruction.op1_register();
           
            // mov reg, [rip]
            let mut inst = Instruction::new();

            inst.set_code( Code::Lea_r64_m );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( reg_0 );
            inst.set_op1_kind( OpKind::Memory );
            inst.set_memory_index( Register::None );
            inst.set_memory_base( Register::RIP );
            inst.set_memory_displacement64( 7 );

            loop_insts.push( inst );

            // add reg, [rax]     
            let mut inst = Instruction::new();

            inst.set_code( Code::Add_r64_rm64 );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( reg_0 );
            inst.set_op1_kind( OpKind::Memory );
            inst.set_memory_index( Register::None );
            inst.set_memory_base( reg_1 );
   
            loop_insts.push( inst );

            // push rdx
            let mut inst = Instruction::new();
    
            inst.set_code( Code::Push_rm64 );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( reg_0 );

            loop_insts.push( inst );

            // mov rdx, rsp
            let mut inst = Instruction::new();
    
            inst.set_code( Code::Mov_r64_rm64 );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( reg_0 );
            inst.set_op1_kind( OpKind::Register );
            inst.set_op1_register( Register::RSP );

            loop_insts.push( inst );
        };

        {
            let mut inst = Instruction::new();
            inst.set_code( Code::Sub_r64_rm64 );
            inst.set_op0_kind( OpKind::Register );
            inst.set_op0_register( Register::RSP );
            inst.set_op1_kind( OpKind::Immediate32to64 );
            inst.set_immediate32to64( 8 );

            loop_insts.push( inst );
        }

        let mut encoder = Encoder::new( 64 );
        for inst in out_inst {
            encoder.encode( &inst, 0 )?;
        }
            
        let mut inst_data = encoder.take_buffer();

        for inst in loop_insts {
            encoder.encode( &inst, 0 )?;
        }
        let mut loop_data = encoder.take_buffer();

        let inst_size = inst_data.len();
        let loop_size = loop_data.len();

        let mut data_buf = Vec::from_iter( 
            std::iter::repeat( 0x90 ).take( THEME_LOAD_LEN.clone() - inst_size - loop_size ) 
        );
        inst_data.append( &mut loop_data );
        inst_data.append( &mut data_buf );

        for i in 0..inst_data.len() {
            text_data[ theme_load_addr + i ] = inst_data[ i ];
        }

        let rip = text_rva + (theme_load_addr + inst_size) as u32;
        for offset in offset_map {
            let offset = (themes_rva as u64 + offset ) - rip as u64;
         
            section.append( &mut (offset + 8).to_le_bytes().to_vec() );
        }

        // move address of jnz
        let addr = bin.scan( &IDAPat::new( THEME_JNZ_PAT.clone() ), Some(".text") ) 
            .map_or(
                Err("Failed to find theme jnz"),
                | addr | Ok( addr )
            )?;

        let jnz_rip = addr as u64 + text_rva as u64 + 3;
        let mut jnz_insts = ThemesPatch::decode_instructions(  &text_data[ addr..addr + THEME_JNZ_LEN.clone() ], jnz_rip );
        
        let original_addr = theme_load_addr + THEME_LOAD_LEN.clone();
        let new_addr = theme_load_addr + inst_size;
        let addr_offset = original_addr - new_addr;

        let jnz_inst = jnz_insts
            .get_mut( 1 )
            .map_or(
                Err("Failed to find instruction"),
                | inst | Ok( inst )
            )?;

        jnz_inst.set_near_branch64( jnz_inst.near_branch64() - addr_offset as u64 );
       
        for inst in jnz_insts {
            encoder.encode( &inst, jnz_rip )?;
        }

        let jnz_data = encoder.take_buffer();
        for i in 0..jnz_data.len() {
            text_data[ addr + i ] = jnz_data[ i ];
        }

        bin.set_section_data( ".themes", section )?;
        bin.set_section_data( ".text", text_data )?;
        
        Ok(())
    }

}