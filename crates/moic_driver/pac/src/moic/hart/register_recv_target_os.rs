#[doc = "Register `register_recv_target_os` reader"]
pub type R = crate::R<RegisterRecvTargetOsSpec>;
#[doc = "Register `register_recv_target_os` writer"]
pub type W = crate::W<RegisterRecvTargetOsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RegisterRecvTargetOsSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Register receive target os.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`register_recv_target_os::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`register_recv_target_os::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegisterRecvTargetOsSpec;
impl crate::RegisterSpec for RegisterRecvTargetOsSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`register_recv_target_os::R`](R) reader structure"]
impl crate::Readable for RegisterRecvTargetOsSpec {}
#[doc = "`write(|w| ..)` method takes [`register_recv_target_os::W`](W) writer structure"]
impl crate::Writable for RegisterRecvTargetOsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets register_recv_target_os to value 0"]
impl crate::Resettable for RegisterRecvTargetOsSpec {
    const RESET_VALUE: u64 = 0;
}
