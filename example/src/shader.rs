// File automatically generated by build.rs.
// Changes made to this file will not be saved.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexInput {
    pub position: [f32; 3],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub color_rgb: [f32; 4],
}
const _: () = assert!(
    std::mem::size_of:: < Uniforms > () == 16, "size of Uniforms does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(Uniforms, color_rgb) == 0,
    "offset of Uniforms.color_rgb does not match WGSL"
);
pub mod bind_groups {
    pub struct BindGroup0(wgpu::BindGroup);
    pub struct BindGroupLayout0<'a> {
        pub color_texture: &'a wgpu::TextureView,
        pub color_sampler: &'a wgpu::Sampler,
    }
    const LAYOUT_DESCRIPTOR0: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    };
    impl BindGroup0 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout0) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0);
            let bind_group = device
                .create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.color_texture,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Sampler(
                                    bindings.color_sampler,
                                ),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
            render_pass.set_bind_group(0, &self.0, &[]);
        }
    }
    pub struct BindGroup1(wgpu::BindGroup);
    pub struct BindGroupLayout1<'a> {
        pub uniforms: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR1: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
    impl BindGroup1 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout1) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1);
            let bind_group = device
                .create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(bindings.uniforms),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
            render_pass.set_bind_group(1, &self.0, &[]);
        }
    }
    pub struct BindGroups<'a> {
        pub bind_group0: &'a BindGroup0,
        pub bind_group1: &'a BindGroup1,
    }
    pub fn set_bind_groups<'a>(
        pass: &mut wgpu::RenderPass<'a>,
        bind_groups: BindGroups<'a>,
    ) {
        bind_groups.bind_group0.set(pass);
        bind_groups.bind_group1.set(pass);
    }
}
pub mod vertex {
    impl super::VertexInput {
        pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 1] = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: memoffset::offset_of!(super::VertexInput, position) as u64,
                shader_location: 0,
            },
        ];
        pub const fn vertex_buffer_layout(
            step_mode: wgpu::VertexStepMode,
        ) -> wgpu::VertexBufferLayout<'static> {
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<super::VertexInput>() as u64,
                step_mode,
                attributes: &super::VertexInput::VERTEX_ATTRIBUTES,
            }
        }
    }
}
pub const ENTRY_VS_MAIN: &str = "vs_main";
pub const ENTRY_FS_MAIN: &str = "fs_main";
pub struct VertexEntry<const N: usize> {
    entry_point: &'static str,
    buffers: [wgpu::VertexBufferLayout<'static>; N],
}
pub fn vertex_state<'a, const N: usize>(
    module: &'a wgpu::ShaderModule,
    entry: &'a VertexEntry<N>,
) -> wgpu::VertexState<'a> {
    wgpu::VertexState {
        module,
        entry_point: entry.entry_point,
        buffers: &entry.buffers,
    }
}
pub fn vs_main_entry(vertex_input: wgpu::VertexStepMode) -> VertexEntry<1> {
    VertexEntry {
        entry_point: ENTRY_VS_MAIN,
        buffers: [VertexInput::vertex_buffer_layout(vertex_input)],
    }
}
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    let source = std::borrow::Cow::Borrowed(include_str!("shader.wgsl"));
    device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(source),
        })
}
pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    device
        .create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &bind_groups::BindGroup0::get_bind_group_layout(device),
                    &bind_groups::BindGroup1::get_bind_group_layout(device),
                ],
                push_constant_ranges: &[],
            },
        )
}
pub struct PipelineStage(wgpu::RenderPipeline);
impl core::ops::Deref for PipelineStage {
    type Target = wgpu::RenderPipeline;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for PipelineStage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl PipelineStage {
    pub fn new(render_pipeline: wgpu::RenderPipeline) -> Self {
        PipelineStage(render_pipeline)
    }
    pub fn set<'s, 'rp>(
        &'s self,
        mut render_pass: wgpu::RenderPass<'rp>,
    ) -> PreparedRenderPass<'rp, NeedsVertexBuffer0, NeedsBindGroup0, NeedsBindGroup1>
    where
        's: 'rp,
    {
        render_pass.set_pipeline(&self.0);
        PreparedRenderPass {
            render_pass,
            type_state: std::marker::PhantomData,
        }
    }
}
pub struct NeedsVertexBuffer0;
pub struct NeedsBindGroup0;
pub struct NeedsBindGroup1;
pub struct Ready;
pub struct PreparedRenderPass<'rp, VB0, BG0, BG1> {
    render_pass: wgpu::RenderPass<'rp>,
    type_state: std::marker::PhantomData<(VB0, BG0, BG1)>,
}
impl<'rp, VB0, BG0, BG1> PreparedRenderPass<'rp, VB0, BG0, BG1> {
    pub fn inner(&mut self) -> &mut wgpu::RenderPass<'rp> {
        &mut self.render_pass
    }
    pub fn set_vertex_buffer_0(
        mut self,
        buffer_slice: wgpu::BufferSlice<'rp>,
    ) -> PreparedRenderPass<'rp, Ready, BG0, BG1> {
        self.render_pass.set_vertex_buffer(0, buffer_slice);
        PreparedRenderPass {
            render_pass: self.render_pass,
            type_state: std::marker::PhantomData,
        }
    }
    pub fn set_bind_group_0(
        mut self,
        bind_group: &'rp bind_groups::BindGroup0,
    ) -> PreparedRenderPass<'rp, VB0, Ready, BG1> {
        bind_group.set(&mut self.render_pass);
        PreparedRenderPass {
            render_pass: self.render_pass,
            type_state: std::marker::PhantomData,
        }
    }
    pub fn set_bind_group_1(
        mut self,
        bind_group: &'rp bind_groups::BindGroup1,
    ) -> PreparedRenderPass<'rp, VB0, BG0, Ready> {
        bind_group.set(&mut self.render_pass);
        PreparedRenderPass {
            render_pass: self.render_pass,
            type_state: std::marker::PhantomData,
        }
    }
}
impl<'rp> PreparedRenderPass<'rp, Ready, Ready, Ready> {
    pub fn into_inner(self) -> wgpu::RenderPass<'rp> {
        self.render_pass
    }
}
impl<'rp> core::ops::Deref for PreparedRenderPass<'rp, Ready, Ready, Ready> {
    type Target = wgpu::RenderPass<'rp>;
    fn deref(&self) -> &Self::Target {
        &self.render_pass
    }
}
impl<'rp> core::ops::DerefMut for PreparedRenderPass<'rp, Ready, Ready, Ready> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.render_pass
    }
}
