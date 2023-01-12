pub(crate) use commands::*;
pub(crate) use helpers::*;
pub(crate) use ash::{vk, Device};

pub mod commands;
pub mod helpers;

/// Alias for `Command::build` for use with `dyn`.
pub trait CommandBuilder {
    fn build(&self, device: &Device, command_buffer: &mut vk::CommandBuffer) -> Result<(), ()>;
}

impl<T: Command> CommandBuilder for T {
    fn build(&self, device: &Device, command_buffer: &mut vk::CommandBuffer) -> Result<(), ()> {
        self.build(device, command_buffer)
    }
}

pub trait Command: Sized {
    type ComputePipeline;
    type ComputeLayout;

    type GraphicsPipeline;
    type GraphicsLayout;

    type RayTracingPipeline;
    type RayTracingLayout;

    type Index;

    fn build(&self, device: &Device, command_buffer: &mut vk::CommandBuffer) -> Result<(), ()>;

    // the Is trait bound isn't quite right, if a pipeline doesn't use, eg. (0, 0) then (0, 0)
    // can be any bound type, however it's very difficult to get that right and this is easy
    fn dispatch<D: DispatchDimensions>(self, dimensions: D) -> Dispatch<Self>
    where
        Self::ComputePipeline: Is<Self::ComputeLayout>,
    {
        Dispatch {
            commands: self,
            dimensions: dimensions.dispatch_dimensions(),
        }
    }

    fn bind_compute_pipeline<P: ComputePipeline>(
        self,
        pipeline: P,
    ) -> BindComputePipeline<Self, P> {
        BindComputePipeline {
            commands: self,
            pipeline,
        }
    }

    fn bind_compute_layout<L: ComputeLayout>(self, layout: L) -> BindComputeLayout<Self, L> {
        BindComputeLayout {
            commands: self,
            layout,
        }
    }

    fn bind_index_buffer<I: IndexBuffer>(self, index_buffer: I) -> BindIndexBuffer<Self, I> {
        BindIndexBuffer {
            commands: self,
            index_buffer,
        }
    }
}

pub trait ComputePipeline {
    fn pipeline(&self) -> vk::Pipeline;
}

pub trait ComputeLayout {
    fn layout(&self) -> vk::PipelineLayout;
    fn first_set(&self) -> u32;
    fn descriptor_sets(&self) -> &[vk::DescriptorSet];
}

pub trait IndexBuffer {
    type Index: VertexIndex;
    fn buffer(&self) -> vk::Buffer;
    fn offset(&self) -> vk::DeviceSize;
}
