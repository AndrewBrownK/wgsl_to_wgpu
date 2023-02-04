// File automatically generated by build.rs.
// Changes made to this file will not be saved.
pub trait Drawable<'d> {
    fn base_vertex(&self) -> i32 {
        return 0;
    }
    fn instance_range(&self) -> core::ops::Range<u32> {
        return 0..1;
    }
    fn get_index_buffer(
        &self,
    ) -> (wgpu::BufferSlice<'d>, wgpu::IndexFormat, core::ops::Range<u32>);
    fn get_bind_group0(&self) -> &'d bind_groups::BindGroup0;
}
pub fn draw<'rp, 'd: 'rp, D: Drawable<'d>>(
    d: D,
    render_pass: &mut wgpu::RenderPass<'rp>,
) {
    let (idx_buffer, idx_fmt, idx_range) = d.get_index_buffer();
    render_pass.set_index_buffer(idx_buffer, idx_fmt);
    d.get_bind_group0().set(render_pass);
    render_pass.draw_indexed(idx_range, d.base_vertex(), d.instance_range());
}
pub mod bind_groups {
    pub struct BindGroup0(wgpu::BindGroup);
    pub struct BindGroupLayout0<'a> {
        pub color_texture: &'a wgpu::TextureView,
        pub color_sampler: &'a wgpu::Sampler,
    }
    pub trait ProvideBindGroup0 {
        fn color_texture(&self) -> &wgpu::TextureView;
        fn color_sampler(&self) -> &wgpu::Sampler;
    }
    const LAYOUT_DESCRIPTOR0: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
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
    pub struct BindGroups {
        pub bind_group0: BindGroup0,
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
                ],
                push_constant_ranges: &[],
            },
        )
}
