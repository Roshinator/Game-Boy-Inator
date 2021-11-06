use super::ram::{Ram};

use usize as Reg;
const REG_A:Reg = 0;
const REG_B:Reg = 1;
const REG_C:Reg = 2;
const REG_D:Reg = 3;
const REG_E:Reg = 4;
const REG_F:Reg = 5;
const REG_H:Reg = 6;
const REG_L:Reg = 7;
//in an AF situation, A is msh, F is lsh, little endian

use u8 as Flag;
const FLAG_Z:Flag = 7;
const FLAG_N:Flag = 6;
const FLAG_H:Flag = 5;
const FLAG_C:Flag = 4;

pub struct Cpu
{
    regs: [u8;8],
    pc: u16,
    sp: u16,
    ram: Ram
}

impl Cpu
{
    //Format [name]_[param1]_[param2]
    //r is a register
    //a means parameter is an address (dereference)

    #[inline]
    fn aux_read_flag(&self, param: Flag) -> bool
    {
        (self.regs[REG_F].to_le() & u8::to_le(1 << param)) > 0
    }

    #[inline]
    fn aux_write_flag(&mut self, param: Flag, data: bool)
    {
        let x = data as u8;
        assert!(x == 0 || x == 1);
        self.regs[REG_F] = self.regs[REG_F].to_le() & u8::to_le(!(!x << param))
    }

    #[inline]
    fn ld_r16_16(&mut self, msh: Reg, lsh: Reg, p2: u16)
    {
        let hl = p2.to_le_bytes();
        self.regs[msh] = hl[0];
        self.regs[lsh] = hl[1];
    }

    #[inline]
    fn ld_r16a_8(&mut self, msh: Reg, lsh: Reg, p2: u8)
    {
        self.ram.write_to_address(u16::from_le_bytes([self.regs[msh], self.regs[lsh]]) as usize, p2);
    }

    #[inline]
    fn ld_r8_8(&mut self, p1: Reg, p2: u8)
    {
        self.regs[p1] = p2;
    }

    #[inline]
    fn ld_r8_r8(&mut self, p1: Reg, p2: Reg)
    {
        self.regs[p1] = self.regs[p2];
    }

    #[inline]
    fn ld_r8_r16a(&mut self, p1: Reg, msh: Reg, lsh: Reg)
    {
        let x = self.ram.read_from_address(u16::from_le_bytes([self.regs[msh], self.regs[lsh]]) as usize);
        self.regs[p1] = x;
    }

    #[inline]
    fn ld_r16a_r8(&mut self, msh: Reg, lsh: Reg, p2: Reg)
    {
        self.ram.write_to_address(u16::from_le_bytes([self.regs[msh], self.regs[lsh]]) as usize, self.regs[p2]);
    }

    fn execute(&mut self)
    {
        match self.pc
        {
            0x0000 => {},//NOP
            0x0001 => {},
            0x0002 => {},
            0x0003 => {},
            0x0004 => {},
            0x0005 => {},
            0x0006 => {},
            0x0007 => {},
            0x0008 => {},
            0x0009 => {},
            0x000A => {},
            0x000B => {},
            0x000C => {},
            0x000D => {},
            0x000E => {},
            0x000F => {},
            0x0010 => {},
            0x0011 => {},
            0x0012 => {},
            0x0013 => {},
            0x0014 => {},
            0x0015 => {},
            0x0016 => {},
            0x0017 => {},
            0x0018 => {},
            0x0019 => {},
            0x001A => {},
            0x001B => {},
            0x001C => {},
            0x001D => {},
            0x001E => {},
            0x001F => {},
            0x0020 => {},
            0x0021 => {},
            0x0022 => {},
            0x0023 => {},
            0x0024 => {},
            0x0025 => {},
            0x0026 => {},
            0x0027 => {},
            0x0028 => {},
            0x0029 => {},
            0x002A => {},
            0x002B => {},
            0x002C => {},
            0x002D => {},
            0x002E => {},
            0x002F => {},
            0x0030 => {},
            0x0031 => {},
            0x0032 => {},
            0x0033 => {},
            0x0034 => {},
            0x0035 => {},
            0x0036 => {},
            0x0037 => {},
            0x0038 => {},
            0x0039 => {},
            0x003A => {},
            0x003B => {},
            0x003C => {},
            0x003D => {},
            0x003E => {},
            0x003F => {},
            0x0040 => {self.ld_r8_r8(REG_B, REG_B)},
            0x0041 => {self.ld_r8_r8(REG_B, REG_C)},
            0x0042 => {self.ld_r8_r8(REG_B, REG_D)},
            0x0043 => {self.ld_r8_r8(REG_B, REG_E)},
            0x0044 => {self.ld_r8_r8(REG_B, REG_H)},
            0x0045 => {self.ld_r8_r8(REG_B, REG_L)},
            0x0046 => {self.ld_r8_r16a(REG_B, REG_H, REG_L)},
            0x0047 => {self.ld_r8_r8(REG_B, REG_A)},
            0x0048 => {self.ld_r8_r8(REG_C, REG_B)},
            0x0049 => {self.ld_r8_r8(REG_C, REG_C)},
            0x004A => {self.ld_r8_r8(REG_C, REG_D)},
            0x004B => {self.ld_r8_r8(REG_C, REG_E)},
            0x004C => {self.ld_r8_r8(REG_C, REG_H)},
            0x004D => {self.ld_r8_r8(REG_C, REG_L)},
            0x004E => {self.ld_r8_r16a(REG_C, REG_H, REG_L)},
            0x004F => {self.ld_r8_r8(REG_C, REG_A)},
            0x0050 => {self.ld_r8_r8(REG_D, REG_B)},
            0x0051 => {self.ld_r8_r8(REG_D, REG_C)},
            0x0052 => {self.ld_r8_r8(REG_D, REG_D)},
            0x0053 => {self.ld_r8_r8(REG_D, REG_E)},
            0x0054 => {self.ld_r8_r8(REG_D, REG_H)},
            0x0055 => {self.ld_r8_r8(REG_D, REG_L)},
            0x0056 => {self.ld_r8_r16a(REG_D, REG_H, REG_L)},
            0x0057 => {self.ld_r8_r8(REG_D, REG_A)},
            0x0058 => {self.ld_r8_r8(REG_E, REG_B)},
            0x0059 => {self.ld_r8_r8(REG_E, REG_C)},
            0x005A => {self.ld_r8_r8(REG_E, REG_D)},
            0x005B => {self.ld_r8_r8(REG_E, REG_E)},
            0x005C => {self.ld_r8_r8(REG_E, REG_H)},
            0x005D => {self.ld_r8_r8(REG_E, REG_L)},
            0x005E => {self.ld_r8_r16a(REG_E, REG_H, REG_L)},
            0x005F => {self.ld_r8_r8(REG_E, REG_A)},
            0x0060 => {self.ld_r8_r8(REG_H, REG_B)},
            0x0061 => {self.ld_r8_r8(REG_H, REG_C)},
            0x0062 => {self.ld_r8_r8(REG_H, REG_D)},
            0x0063 => {self.ld_r8_r8(REG_H, REG_E)},
            0x0064 => {self.ld_r8_r8(REG_H, REG_H)},
            0x0065 => {self.ld_r8_r8(REG_H, REG_L)},
            0x0066 => {self.ld_r8_r16a(REG_H, REG_H, REG_L)},
            0x0067 => {self.ld_r8_r8(REG_H, REG_A)},
            0x0068 => {self.ld_r8_r8(REG_L, REG_B)},
            0x0069 => {self.ld_r8_r8(REG_L, REG_C)},
            0x006A => {self.ld_r8_r8(REG_L, REG_D)},
            0x006B => {self.ld_r8_r8(REG_L, REG_E)},
            0x006C => {self.ld_r8_r8(REG_L, REG_H)},
            0x006D => {self.ld_r8_r8(REG_L, REG_L)},
            0x006E => {self.ld_r8_r16a(REG_L, REG_H, REG_L)},
            0x006F => {self.ld_r8_r8(REG_L, REG_A)},
            0x0070 => {self.ld_r16a_r8(REG_H, REG_L, REG_B)},
            0x0071 => {self.ld_r16a_r8(REG_H, REG_L, REG_C)},
            0x0072 => {self.ld_r16a_r8(REG_H, REG_L, REG_D)},
            0x0073 => {self.ld_r16a_r8(REG_H, REG_L, REG_E)},
            0x0074 => {self.ld_r16a_r8(REG_H, REG_L, REG_H)},
            0x0075 => {self.ld_r16a_r8(REG_H, REG_L, REG_L)},
            0x0076 => {},
            0x0077 => {self.ld_r16a_r8(REG_H, REG_L, REG_A)},
            0x0078 => {self.ld_r8_r8(REG_A, REG_B)},
            0x0079 => {self.ld_r8_r8(REG_A, REG_C)},
            0x007A => {self.ld_r8_r8(REG_A, REG_D)},
            0x007B => {self.ld_r8_r8(REG_A, REG_E)},
            0x007C => {self.ld_r8_r8(REG_A, REG_H)},
            0x007D => {self.ld_r8_r8(REG_A, REG_L)},
            0x007E => {self.ld_r8_r16a(REG_A, REG_H, REG_L)},
            0x007F => {self.ld_r8_r8(REG_A, REG_A)},
            0x0080 => {},
            0x0081 => {},
            0x0082 => {},
            0x0083 => {},
            0x0084 => {},
            0x0085 => {},
            0x0086 => {},
            0x0087 => {},
            0x0088 => {},
            0x0089 => {},
            0x008A => {},
            0x008B => {},
            0x008C => {},
            0x008D => {},
            0x008E => {},
            0x008F => {},
            0x0090 => {},
            0x0091 => {},
            0x0092 => {},
            0x0093 => {},
            0x0094 => {},
            0x0095 => {},
            0x0096 => {},
            0x0097 => {},
            0x0098 => {},
            0x0099 => {},
            0x009A => {},
            0x009B => {},
            0x009C => {},
            0x009D => {},
            0x009E => {},
            0x009F => {},
            0x00A0 => {},
            0x00A1 => {},
            0x00A2 => {},
            0x00A3 => {},
            0x00A4 => {},
            0x00A5 => {},
            0x00A6 => {},
            0x00A7 => {},
            0x00A8 => {},
            0x00A9 => {},
            0x00AA => {},
            0x00AB => {},
            0x00AC => {},
            0x00AD => {},
            0x00AE => {},
            0x00AF => {},
            0x00B0 => {},
            0x00B1 => {},
            0x00B2 => {},
            0x00B3 => {},
            0x00B4 => {},
            0x00B5 => {},
            0x00B6 => {},
            0x00B7 => {},
            0x00B8 => {},
            0x00B9 => {},
            0x00BA => {},
            0x00BB => {},
            0x00BC => {},
            0x00BD => {},
            0x00BE => {},
            0x00BF => {},
            0x00C0 => {},
            0x00C1 => {},
            0x00C2 => {},
            0x00C3 => {},
            0x00C4 => {},
            0x00C5 => {},
            0x00C6 => {},
            0x00C7 => {},
            0x00C8 => {},
            0x00C9 => {},
            0x00CA => {},
            0x00CB => {},
            0x00CC => {},
            0x00CD => {},
            0x00CE => {},
            0x00CF => {},
            0x00D0 => {},
            0x00D1 => {},
            0x00D2 => {},
            0x00D3 => {},
            0x00D4 => {},
            0x00D5 => {},
            0x00D6 => {},
            0x00D7 => {},
            0x00D8 => {},
            0x00D9 => {},
            0x00DA => {},
            0x00DB => {},
            0x00DC => {},
            0x00DD => {},
            0x00DE => {},
            0x00DF => {},
            0x00E0 => {},
            0x00E1 => {},
            0x00E2 => {},
            0x00E3 => {},
            0x00E4 => {},
            0x00E5 => {},
            0x00E6 => {},
            0x00E7 => {},
            0x00E8 => {},
            0x00E9 => {},
            0x00EA => {},
            0x00EB => {},
            0x00EC => {},
            0x00ED => {},
            0x00EE => {},
            0x00EF => {},
            0x00F0 => {},
            0x00F1 => {},
            0x00F2 => {},
            0x00F3 => {},
            0x00F4 => {},
            0x00F5 => {},
            0x00F6 => {},
            0x00F7 => {},
            0x00F8 => {},
            0x00F9 => {},
            0x00FA => {},
            0x00FB => {},
            0x00FC => {},
            0x00FD => {},
            0x00FE => {},
            0x00FF => {},
            0xCB00 => {},
            0xCB01 => {},
            0xCB02 => {},
            0xCB03 => {},
            0xCB04 => {},
            0xCB05 => {},
            0xCB06 => {},
            0xCB07 => {},
            0xCB08 => {},
            0xCB09 => {},
            0xCB0A => {},
            0xCB0B => {},
            0xCB0C => {},
            0xCB0D => {},
            0xCB0E => {},
            0xCB0F => {},
            0xCB10 => {},
            0xCB11 => {},
            0xCB12 => {},
            0xCB13 => {},
            0xCB14 => {},
            0xCB15 => {},
            0xCB16 => {},
            0xCB17 => {},
            0xCB18 => {},
            0xCB19 => {},
            0xCB1A => {},
            0xCB1B => {},
            0xCB1C => {},
            0xCB1D => {},
            0xCB1E => {},
            0xCB1F => {},
            0xCB20 => {},
            0xCB21 => {},
            0xCB22 => {},
            0xCB23 => {},
            0xCB24 => {},
            0xCB25 => {},
            0xCB26 => {},
            0xCB27 => {},
            0xCB28 => {},
            0xCB29 => {},
            0xCB2A => {},
            0xCB2B => {},
            0xCB2C => {},
            0xCB2D => {},
            0xCB2E => {},
            0xCB2F => {},
            0xCB30 => {},
            0xCB31 => {},
            0xCB32 => {},
            0xCB33 => {},
            0xCB34 => {},
            0xCB35 => {},
            0xCB36 => {},
            0xCB37 => {},
            0xCB38 => {},
            0xCB39 => {},
            0xCB3A => {},
            0xCB3B => {},
            0xCB3C => {},
            0xCB3D => {},
            0xCB3E => {},
            0xCB3F => {},
            0xCB40 => {},
            0xCB41 => {},
            0xCB42 => {},
            0xCB43 => {},
            0xCB44 => {},
            0xCB45 => {},
            0xCB46 => {},
            0xCB47 => {},
            0xCB48 => {},
            0xCB49 => {},
            0xCB4A => {},
            0xCB4B => {},
            0xCB4C => {},
            0xCB4D => {},
            0xCB4E => {},
            0xCB4F => {},
            0xCB50 => {},
            0xCB51 => {},
            0xCB52 => {},
            0xCB53 => {},
            0xCB54 => {},
            0xCB55 => {},
            0xCB56 => {},
            0xCB57 => {},
            0xCB58 => {},
            0xCB59 => {},
            0xCB5A => {},
            0xCB5B => {},
            0xCB5C => {},
            0xCB5D => {},
            0xCB5E => {},
            0xCB5F => {},
            0xCB60 => {},
            0xCB61 => {},
            0xCB62 => {},
            0xCB63 => {},
            0xCB64 => {},
            0xCB65 => {},
            0xCB66 => {},
            0xCB67 => {},
            0xCB68 => {},
            0xCB69 => {},
            0xCB6A => {},
            0xCB6B => {},
            0xCB6C => {},
            0xCB6D => {},
            0xCB6E => {},
            0xCB6F => {},
            0xCB70 => {},
            0xCB71 => {},
            0xCB72 => {},
            0xCB73 => {},
            0xCB74 => {},
            0xCB75 => {},
            0xCB76 => {},
            0xCB77 => {},
            0xCB78 => {},
            0xCB79 => {},
            0xCB7A => {},
            0xCB7B => {},
            0xCB7C => {},
            0xCB7D => {},
            0xCB7E => {},
            0xCB7F => {},
            0xCB80 => {},
            0xCB81 => {},
            0xCB82 => {},
            0xCB83 => {},
            0xCB84 => {},
            0xCB85 => {},
            0xCB86 => {},
            0xCB87 => {},
            0xCB88 => {},
            0xCB89 => {},
            0xCB8A => {},
            0xCB8B => {},
            0xCB8C => {},
            0xCB8D => {},
            0xCB8E => {},
            0xCB8F => {},
            0xCB90 => {},
            0xCB91 => {},
            0xCB92 => {},
            0xCB93 => {},
            0xCB94 => {},
            0xCB95 => {},
            0xCB96 => {},
            0xCB97 => {},
            0xCB98 => {},
            0xCB99 => {},
            0xCB9A => {},
            0xCB9B => {},
            0xCB9C => {},
            0xCB9D => {},
            0xCB9E => {},
            0xCB9F => {},
            0xCBA0 => {},
            0xCBA1 => {},
            0xCBA2 => {},
            0xCBA3 => {},
            0xCBA4 => {},
            0xCBA5 => {},
            0xCBA6 => {},
            0xCBA7 => {},
            0xCBA8 => {},
            0xCBA9 => {},
            0xCBAA => {},
            0xCBAB => {},
            0xCBAC => {},
            0xCBAD => {},
            0xCBAE => {},
            0xCBAF => {},
            0xCBB0 => {},
            0xCBB1 => {},
            0xCBB2 => {},
            0xCBB3 => {},
            0xCBB4 => {},
            0xCBB5 => {},
            0xCBB6 => {},
            0xCBB7 => {},
            0xCBB8 => {},
            0xCBB9 => {},
            0xCBBA => {},
            0xCBBB => {},
            0xCBBC => {},
            0xCBBD => {},
            0xCBBE => {},
            0xCBBF => {},
            0xCBC0 => {},
            0xCBC1 => {},
            0xCBC2 => {},
            0xCBC3 => {},
            0xCBC4 => {},
            0xCBC5 => {},
            0xCBC6 => {},
            0xCBC7 => {},
            0xCBC8 => {},
            0xCBC9 => {},
            0xCBCA => {},
            0xCBCB => {},
            0xCBCC => {},
            0xCBCD => {},
            0xCBCE => {},
            0xCBCF => {},
            0xCBD0 => {},
            0xCBD1 => {},
            0xCBD2 => {},
            0xCBD3 => {},
            0xCBD4 => {},
            0xCBD5 => {},
            0xCBD6 => {},
            0xCBD7 => {},
            0xCBD8 => {},
            0xCBD9 => {},
            0xCBDA => {},
            0xCBDB => {},
            0xCBDC => {},
            0xCBDD => {},
            0xCBDE => {},
            0xCBDF => {},
            0xCBE0 => {},
            0xCBE1 => {},
            0xCBE2 => {},
            0xCBE3 => {},
            0xCBE4 => {},
            0xCBE5 => {},
            0xCBE6 => {},
            0xCBE7 => {},
            0xCBE8 => {},
            0xCBE9 => {},
            0xCBEA => {},
            0xCBEB => {},
            0xCBEC => {},
            0xCBED => {},
            0xCBEE => {},
            0xCBEF => {},
            0xCBF0 => {},
            0xCBF1 => {},
            0xCBF2 => {},
            0xCBF3 => {},
            0xCBF4 => {},
            0xCBF5 => {},
            0xCBF6 => {},
            0xCBF7 => {},
            0xCBF8 => {},
            0xCBF9 => {},
            0xCBFA => {},
            0xCBFB => {},
            0xCBFC => {},
            0xCBFD => {},
            0xCBFE => {},
            0xCBFF => {},
            _ => panic!("Tried to execute invalid instruction")
        }
    }
}