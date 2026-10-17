#[doc = "Register `UTX_IR_POSITION` reader"]
pub type R = crate::R<UtxIrPositionSpec>;
#[doc = "Register `UTX_IR_POSITION` writer"]
pub type W = crate::W<UtxIrPositionSpec>;
#[doc = "Field `cr_utx_ir_pos_s` reader - "]
pub type CrUtxIrPosSR = crate::FieldReader<u16>;
#[doc = "Field `cr_utx_ir_pos_s` writer - "]
pub type CrUtxIrPosSW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cr_utx_ir_pos_p` reader - "]
pub type CrUtxIrPosPR = crate::FieldReader<u16>;
#[doc = "Field `cr_utx_ir_pos_p` writer - "]
pub type CrUtxIrPosPW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_s(&self) -> CrUtxIrPosSR {
        CrUtxIrPosSR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_p(&self) -> CrUtxIrPosPR {
        CrUtxIrPosPR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_s(&mut self) -> CrUtxIrPosSW<'_, UtxIrPositionSpec> {
        CrUtxIrPosSW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_p(&mut self) -> CrUtxIrPosPW<'_, UtxIrPositionSpec> {
        CrUtxIrPosPW::new(self, 16)
    }
}
#[doc = "UART TX IR Pulse Position\n\nYou can [`read`](crate::Reg::read) this register and get [`utx_ir_position::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utx_ir_position::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtxIrPositionSpec;
impl crate::RegisterSpec for UtxIrPositionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utx_ir_position::R`](R) reader structure"]
impl crate::Readable for UtxIrPositionSpec {}
#[doc = "`write(|w| ..)` method takes [`utx_ir_position::W`](W) writer structure"]
impl crate::Writable for UtxIrPositionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UTX_IR_POSITION to value 0x009f_0070"]
impl crate::Resettable for UtxIrPositionSpec {
    const RESET_VALUE: u32 = 0x009f_0070;
}
