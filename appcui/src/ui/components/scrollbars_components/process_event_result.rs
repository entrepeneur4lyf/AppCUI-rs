use std::ops::BitOrAssign;

const REPAINT_BIT: u8 = 1;
const UPDATE_VALUE_BIT: u8 = 2;
const PROCESSED_BY_COMPONENT_BIT: u8 = 4;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct ProcessEventResult {
    value: u8,
}
impl ProcessEventResult {
    #[allow(non_upper_case_globals)]
    pub(crate) const Repaint: ProcessEventResult = ProcessEventResult {
        value: REPAINT_BIT | PROCESSED_BY_COMPONENT_BIT,
    };
    #[allow(non_upper_case_globals)]
    pub(crate) const Update: ProcessEventResult = ProcessEventResult {
        value: UPDATE_VALUE_BIT | PROCESSED_BY_COMPONENT_BIT,
    };
    #[allow(non_upper_case_globals)]
    pub(crate) const PassToControl: ProcessEventResult = ProcessEventResult { value: 0 };
    #[allow(non_upper_case_globals)]
    pub(crate) const PassToControlAndRepaint: ProcessEventResult = ProcessEventResult { value: REPAINT_BIT };
    #[allow(non_upper_case_globals)]
    pub(crate) const Processed: ProcessEventResult = ProcessEventResult {
        value: PROCESSED_BY_COMPONENT_BIT,
    };

    #[inline(always)]
    pub(crate) fn should_repaint(&self) -> bool {
        (self.value & (REPAINT_BIT | UPDATE_VALUE_BIT)) != 0
    }
    #[inline(always)]
    pub(crate) fn should_update(&self) -> bool {
        (self.value & UPDATE_VALUE_BIT) != 0
    }
}
impl BitOrAssign for ProcessEventResult {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value
    }
}
