#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum EntryReg{VbarEl1,VbarEl2,VbarEl3,CptrEl2,CpacrEl1,ScrEl3,CptrEl3,CntfrqEl0,SctlrEl1,SctlrEl2,SctlrEl3,HcrEl2}
pub trait EntryCpuIo{fn read(&mut self,r:EntryReg)->u64;fn write(&mut self,r:EntryReg,v:u64);fn clear_daif_a(&mut self);fn isb(&mut self);}
pub fn apply_entry_cpu_state<I:EntryCpuIo>(io:&mut I,current_el:u64,timer_hz:u64){
    match current_el{
        8=>{io.write(EntryReg::VbarEl2,crate::stage8::VECTOR_BASE);io.write(EntryReg::CptrEl2,0x33ff)},
        4=>{io.write(EntryReg::VbarEl1,crate::stage8::VECTOR_BASE);io.write(EntryReg::CpacrEl1,0x300000)},
        _=>{io.write(EntryReg::VbarEl3,crate::stage8::VECTOR_BASE);let scr=io.read(EntryReg::ScrEl3);io.write(EntryReg::ScrEl3,scr|0xF);io.write(EntryReg::CptrEl3,0);io.write(EntryReg::CntfrqEl0,timer_hz)}
    }
    match current_el{
        8=>{let s=io.read(EntryReg::SctlrEl2);io.write(EntryReg::SctlrEl2,s|0x1000);let h=io.read(EntryReg::HcrEl2);io.write(EntryReg::HcrEl2,h|0x08000020)},
        4=>{let s=io.read(EntryReg::SctlrEl1);io.write(EntryReg::SctlrEl1,s|0x1000)},
        _=>{let s=io.read(EntryReg::SctlrEl3);io.write(EntryReg::SctlrEl3,s|0x1000)}
    }
    io.clear_daif_a();io.isb();
}
#[cfg(test)]mod tests{use super::*;struct M{h:u64,s:u64,v:u64,n:u8}impl EntryCpuIo for M{fn read(&mut self,r:EntryReg)->u64{match r{EntryReg::HcrEl2=>self.h,EntryReg::ScrEl3=>self.s,_=>0}}fn write(&mut self,r:EntryReg,v:u64){self.n+=1;match r{EntryReg::HcrEl2=>self.h=v,EntryReg::VbarEl1|EntryReg::VbarEl2|EntryReg::VbarEl3=>self.v=v,_=>{}}}fn clear_daif_a(&mut self){}fn isb(&mut self){}}#[test]fn entry_el2(){let mut m=M{h:0,s:0,v:0,n:0};apply_entry_cpu_state(&mut m,8,24_000_000);assert_eq!(m.v,crate::stage8::VECTOR_BASE);assert_eq!(m.h&0x08000020,0x08000020);assert!(m.n>=4);}}
