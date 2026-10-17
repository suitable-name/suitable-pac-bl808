#[doc = "Register `SPI_BACKUP_IO_EN` reader"]
pub type R = crate::R<SpiBackupIoEnSpec>;
#[doc = "Register `SPI_BACKUP_IO_EN` writer"]
pub type W = crate::W<SpiBackupIoEnSpec>;
#[doc = "Field `backup_io_en` reader - "]
pub type BackupIoEnR = crate::BitReader;
#[doc = "Field `backup_io_en` writer - "]
pub type BackupIoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn backup_io_en(&self) -> BackupIoEnR {
        BackupIoEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn backup_io_en(&mut self) -> BackupIoEnW<'_, SpiBackupIoEnSpec> {
        BackupIoEnW::new(self, 0)
    }
}
#[doc = "SPI Backup IO Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_backup_io_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_backup_io_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiBackupIoEnSpec;
impl crate::RegisterSpec for SpiBackupIoEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_backup_io_en::R`](R) reader structure"]
impl crate::Readable for SpiBackupIoEnSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_backup_io_en::W`](W) writer structure"]
impl crate::Writable for SpiBackupIoEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_BACKUP_IO_EN to value 0"]
impl crate::Resettable for SpiBackupIoEnSpec {}
