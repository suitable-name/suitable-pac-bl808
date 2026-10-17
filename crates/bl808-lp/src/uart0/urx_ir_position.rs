#[doc = "Register `URX_IR_POSITION` reader"]
pub type R = crate::R<UrxIrPositionSpec>;
#[doc = "Register `URX_IR_POSITION` writer"]
pub type W = crate::W<UrxIrPositionSpec>;
#[doc = "Field `cr_urx_ir_pos_s` reader - "]
pub type CrUrxIrPosSR = crate::FieldReader<u16>;
#[doc = "Field `cr_urx_ir_pos_s` writer - "]
pub type CrUrxIrPosSW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_urx_ir_pos_s(&self) -> CrUrxIrPosSR {
        CrUrxIrPosSR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_urx_ir_pos_s(&mut self) -> CrUrxIrPosSW<'_, UrxIrPositionSpec> {
        CrUrxIrPosSW::new(self, 0)
    }
}
#[doc = "UART RX IR Pulse Position\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_ir_position::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_ir_position::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrxIrPositionSpec;
impl crate::RegisterSpec for UrxIrPositionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urx_ir_position::R`](R) reader structure"]
impl crate::Readable for UrxIrPositionSpec {}
#[doc = "`write(|w| ..)` method takes [`urx_ir_position::W`](W) writer structure"]
impl crate::Writable for UrxIrPositionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets URX_IR_POSITION to value 0x6f"]
impl crate::Resettable for UrxIrPositionSpec {
    const RESET_VALUE: u32 = 0x6f;
}
