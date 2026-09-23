#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum EntryReg{CpacrEl1,ScrEl3,CptrEl3,CptrEl2,HcrEl2,SctlrEl1,SctlrEl2,SctlrEl3,CntfrqEl0}
pub trait EntryCpuIo{fn read(&mut self,r:EntryReg)->u64;fn write(&mut self,r:EntryReg,v:u64);fn clear_daif_a(&mut self);fn isb(&mut self);}
pub fn apply_entry_cpu_state<I:EntryCpuIo>(io:&mut I,current_el:u64,id_aa64pfr0:u64,timer_hz:u64){
    if current_el>8{let scr=io.read(EntryReg::ScrEl3);io.write(EntryReg::ScrEl3,scr|0xF);io.write(EntryReg::CptrEl3,0)}
    else if current_el==8{let h=io.read(EntryReg::HcrEl2);if h&0x4_0000_0000==0{io.write(EntryReg::HcrEl2,h|0x20);io.write(EntryReg::CptrEl2,0x33ff)}else{io.write(EntryReg::CpacrEl1,0x300000)}}
    else{io.write(EntryReg::CpacrEl1,0x300000)}
    match current_el{
        12=>{let s=io.read(EntryReg::SctlrEl3);io.write(EntryReg::SctlrEl3,s|0x1000)},
        8=>{let s=io.read(EntryReg::SctlrEl2);io.write(EntryReg::SctlrEl2,s|0x1000);let h=io.read(EntryReg::HcrEl2);io.write(EntryReg::HcrEl2,h|0x08000020)},
        _=>{let s=io.read(EntryReg::SctlrEl1);io.write(EntryReg::SctlrEl1,s|0x1000)}
    }
    io.clear_daif_a();io.isb();
    let set_timer=if current_el>8{true}else if current_el==8{id_aa64pfr0&0xF000==0}else{id_aa64pfr0&0xFF00==0};
    if set_timer{io.write(EntryReg::CntfrqEl0,timer_hz)}
    io.isb();
}
#[cfg(test)]mod tests{use super::*;struct M{h:u64,f:u64}impl EntryCpuIo for M{fn read(&mut self,r:EntryReg)->u64{if r==EntryReg::HcrEl2{self.h}else{0}}fn write(&mut self,r:EntryReg,v:u64){if r==EntryReg::HcrEl2{self.h=v}if r==EntryReg::CntfrqEl0{self.f=v}}fn clear_daif_a(&mut self){}fn isb(&mut self){}}#[test]fn entry(){let mut m=M{h:0,f:0};apply_entry_cpu_state(&mut m,8,0,24_000_000);assert_eq!(m.h&0x08000020,0x08000020);assert_eq!(m.f,24_000_000);}}
