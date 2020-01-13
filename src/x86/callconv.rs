use crate::core::{
  globals::{
    AsmJitError,
    Error,
  },
  callconv::{
    CallConv,
    CallConvVariants,
    Flags,
    Strategy,
  },
  arch::ArchVariants
};

fn call_conv_common_init(cc: &mut CallConv) {
  cc.set_natural_stack_alignment(4);
  cc.set_arch(ArchVariants::X86);
  cc.set_preserved_regs(Reg::kGroupGp, Support::bitMask(Gp::kIdBx, Gp::kIdSp, Gp::kIdBp, Gp::kIdSi, Gp::kIdDi));
}
  
pub fn init(cc: &mut CallConv, ccId: CallConvVariants) {
  let kGroupGp: u32 = Reg::kGroupGp;
  let kGroupVec: u32 = Reg::kGroupVec;
  let kGroupMm: u32 = Reg::kGroupMm;
  let kGroupKReg: u32 = Reg::kGroupKReg;
  
  let kZax: u32 = Gp::kIdAx;
  let kZbx: u32 = Gp::kIdBx;
  let kZcx: u32 = Gp::kIdCx;
  let kZdx: u32 = Gp::kIdDx;
  let kZsp: u32 = Gp::kIdSp;
  let kZbp: u32 = Gp::kIdBp;
  let kZsi: u32 = Gp::kIdSi;
  let kZdi: u32 = Gp::kIdDi;
  
  match ccId {
    CallConvVariants::X86StdCall => {
      cc.set_flags(Flags::CALLEE_POPS_STACK);
      call_conv_common_init(cc);
    }
    CallConvVariants::X86StdCall => {
      cc.set_flags(Flags::CALLEE_POPS_STACK);
      cc.set_passed_order(kGroupGp, kZcx);
      call_conv_common_init(cc);
    }
  
    CallConvVariants::X86MsFastCall 
    | CallConvVariants::X86GccFastCall => {
      cc.set_flags(Flags::CALLEE_POPS_STACK);
      cc.set_passed_order(kGroupGp, kZcx, kZdx);
      call_conv_common_init(cc);
    }
  
    CallConvVariants::X86GccRegParm1 => {
      cc.set_passed_order(kGroupGp, kZax);
      call_conv_common_init(cc);
    }
  
    CallConvVariants::X86GccRegParm2 => {
      cc.set_passed_order(kGroupGp, kZax, kZdx);
      call_conv_common_init(cc);
    }
  
    CallConvVariants::X86GccRegParm3 => {
      cc.set_passed_order(kGroupGp, kZax, kZdx, kZcx);
      call_conv_common_init(cc);
    }
  
    CallConvVariants::X86CDecl => {
      call_conv_common_init(cc);
    }
  
    CallConvVariants::X86Win64 => {
      cc.set_arch(ArchVariants::X64);
      cc.set_strategy(Strategy::Win64);
      cc.set_flags(Flags::PASS_FLOATS_BY_VEC | Flags::INDIRECT_VEC_ARGS);
      cc.set_natural_stack_alignment(16);
      cc.set_spill_zone_size(32);
      cc.set_passed_order(kGroupGp, kZcx, kZdx, 8, 9);
      cc.set_passed_order(kGroupVec, 0, 1, 2, 3);
      cc.set_preserved_regs(kGroupGp, Support::bitMask(kZbx, kZsp, kZbp, kZsi, kZdi, 12, 13, 14, 15));
      cc.set_preserved_regs(kGroupVec, Support::bitMask(6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
    }
  
    CallConvVariants::X86SysV64 => {
      cc.set_arch(ArchVariants::X64);
      cc.set_flags(Flags::PASS_FLOATS_BY_VEC);
      cc.set_natural_stack_alignment(16);
      cc.set_red_zone_size(128);
      cc.set_passed_order(kGroupGp, kZdi, kZsi, kZdx, kZcx, 8, 9);
      cc.set_passed_order(kGroupVec, 0, 1, 2, 3, 4, 5, 6, 7);
      cc.set_preserved_regs(kGroupGp, Support::bitMask(kZbx, kZsp, kZbp, 12, 13, 14, 15));
    }
  
    CallConvVariants::X86LightCall2
    | CallConvVariants::X86LightCall3
    | CallConvVariants::X86LightCall4 => {
      uint32_t n = (ccId - CallConv::kIdX86LightCall2) + 2;
  
      cc.set_arch_type(ArchInfo::kIdX86);
      cc.set_flags(CallConv::kFlagPassFloatsByVec);
      cc.set_natural_stack_alignment(16);
      cc.set_passed_order(kGroupGp, kZax, kZdx, kZcx, kZsi, kZdi);
      cc.set_passed_order(kGroupMm, 0, 1, 2, 3, 4, 5, 6, 7);
      cc.set_passed_order(kGroupVec, 0, 1, 2, 3, 4, 5, 6, 7);
      cc.set_passed_order(kGroupKReg, 0, 1, 2, 3, 4, 5, 6, 7);
  
      cc.set_preserved_regs(kGroupGp  , Support::lsbMask<uint32_t>(8));
      cc.set_preserved_regs(kGroupVec , Support::lsbMask<uint32_t>(8) & ~Support::lsbMask<uint32_t>(n));
    }
  
    CallConvVariants::X64LightCall2
    | CallConvVariants::X64LightCall3
    | CallConvVariants::X64LightCall4 => {
      uint32_t n = (ccId - CallConv::kIdX64LightCall2) + 2;
  
      cc.set_arch(ArchInfo::kIdX64);
      cc.set_flags(Flags::PASS_FLOATS_BY_VEC);
      cc.set_natural_stack_alignment(16);
      cc.set_passed_order(kGroupGp, kZax, kZdx, kZcx, kZsi, kZdi);
      cc.set_passed_order(kGroupMm, 0, 1, 2, 3, 4, 5, 6, 7);
      cc.set_passed_order(kGroupVec, 0, 1, 2, 3, 4, 5, 6, 7);
      cc.set_passed_order(kGroupKReg, 0, 1, 2, 3, 4, 5, 6, 7);
  
      cc.set_preserved_regs(kGroupGp  , Support::lsbMask<uint32_t>(16));
      cc.set_preserved_regs(kGroupVec ,~Support::lsbMask<uint32_t>(n));
    }
  }
  
  cc.set_call_conv(ccId);
}