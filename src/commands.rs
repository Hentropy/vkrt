use crate::*;

pub struct Dispatch<C> {
    pub(crate) commands: C,
    pub(crate) dimensions: [u32; 3],
}

impl<C: Command> Command for Dispatch<C> {
    type ComputePipeline = C::ComputePipeline;
    type ComputeLayout = C::ComputeLayout;

    type GraphicsPipeline = C::GraphicsPipeline;
    type GraphicsLayout = C::GraphicsLayout;

    type RayTracingPipeline = C::RayTracingPipeline;
    type RayTracingLayout = C::RayTracingLayout;

    type Index = C::Index;

    fn build(
        &self,
        device: &crate::Device,
        command_buffer: &mut crate::vk::CommandBuffer,
    ) -> Result<(), ()> {
        self.commands.build(device, command_buffer)?;
        let [group_count_x, group_count_y, group_count_z] = self.dimensions;
        unsafe {
            device.cmd_dispatch(*command_buffer, group_count_x, group_count_y, group_count_z);
        }
        Ok(())
    }
}

pub struct BindComputePipeline<C, P> {
    pub(crate) commands: C,
    pub(crate) pipeline: P,
}

impl<C: Command, P: ComputePipeline> Command for BindComputePipeline<C, P> {
    type ComputePipeline = P;
    type ComputeLayout = C::ComputeLayout;

    type GraphicsPipeline = C::GraphicsPipeline;
    type GraphicsLayout = C::GraphicsLayout;

    type RayTracingPipeline = C::RayTracingPipeline;
    type RayTracingLayout = C::RayTracingLayout;

    type Index = C::Index;

    fn build(&self, device: &Device, command_buffer: &mut vk::CommandBuffer) -> Result<(), ()> {
        self.commands.build(device, command_buffer)?;
        unsafe {
            device.cmd_bind_pipeline(
                *command_buffer,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline.pipeline(),
            );
        }
        Ok(())
    }
}

pub struct BindComputeLayout<C, L> {
    pub(crate) commands: C,
    pub(crate) layout: L,
}

impl<C: Command, L: ComputeLayout> Command for BindComputeLayout<C, L> {
    type ComputePipeline = C::ComputePipeline;
    type ComputeLayout = L;

    type GraphicsPipeline = C::GraphicsPipeline;
    type GraphicsLayout = C::GraphicsLayout;

    type RayTracingPipeline = C::RayTracingPipeline;
    type RayTracingLayout = C::RayTracingLayout;

    type Index = C::Index;

    fn build(&self, device: &Device, command_buffer: &mut vk::CommandBuffer) -> Result<(), ()> {
        self.commands.build(device, command_buffer)?;
        unsafe {
            device.cmd_bind_descriptor_sets(
                *command_buffer,
                vk::PipelineBindPoint::COMPUTE,
                self.layout.layout(),
                self.layout.first_set(),
                self.layout.descriptor_sets(),
                &[],
            );
        }
        Ok(())
    }
}

pub struct BindIndexBuffer<C, I> {
    pub(crate) commands: C,
    pub(crate) index_buffer: I,
}

impl<C: Command, I: IndexBuffer> Command for BindIndexBuffer<C, I> {
    type ComputePipeline = C::ComputePipeline;
    type ComputeLayout = C::ComputeLayout;

    type GraphicsPipeline = C::GraphicsPipeline;
    type GraphicsLayout = C::GraphicsLayout;

    type RayTracingPipeline = C::RayTracingPipeline;
    type RayTracingLayout = C::RayTracingLayout;

    type Index = I::Index;

    fn build(&self, device: &Device, command_buffer: &mut vk::CommandBuffer) -> Result<(), ()> {
        self.commands.build(device, command_buffer)?;
        unsafe {
            device.cmd_bind_index_buffer(
                *command_buffer,
                self.index_buffer.buffer(),
                self.index_buffer.offset(),
                I::Index::INDEX_TYPE,
            );
        }
        Ok(())
    }
}
