pub trait DispatchDimensions {
    fn dispatch_dimensions(&self) -> [u32; 3];
}

impl DispatchDimensions for [u32; 1] {
    fn dispatch_dimensions(&self) -> [u32; 3] {
        [self[0], 1, 1]
    }
}

impl DispatchDimensions for [u32; 2] {
    fn dispatch_dimensions(&self) -> [u32; 3] {
        [self[0], self[1], 1]
    }
}

impl DispatchDimensions for [u32; 3] {
    fn dispatch_dimensions(&self) -> [u32; 3] {
        *self
    }
}

impl DispatchDimensions for u32 {
    fn dispatch_dimensions(&self) -> [u32; 3] {
        [*self, 1, 1]
    }
}

pub trait IndexType {
    const INDEX_TYPE: crate::vk::IndexType;
}

// used for ray tracing, not used for vertex indexing
impl IndexType for () {
    const INDEX_TYPE: crate::vk::IndexType = crate::vk::IndexType::NONE_KHR;
}

impl IndexType for u8 {
    const INDEX_TYPE: crate::vk::IndexType = crate::vk::IndexType::UINT8_EXT;
}

impl IndexType for u16 {
    const INDEX_TYPE: crate::vk::IndexType = crate::vk::IndexType::UINT16;
}

impl IndexType for u32 {
    const INDEX_TYPE: crate::vk::IndexType = crate::vk::IndexType::UINT32;
}

pub trait VertexIndex: IndexType {}
impl VertexIndex for u8 {}
impl VertexIndex for u16 {}
impl VertexIndex for u32 {}

mod sealed {
    pub trait Is {
        type Equal;
    }
    impl<T> Is for T {
        type Equal = T;
    }
}

pub trait Is<T>: sealed::Is<Equal = T> {}
impl<T> Is<T> for T {}
