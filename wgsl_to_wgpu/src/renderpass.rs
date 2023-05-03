use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

/// The enhanced render pass is a wrapper around RenderPass that gives us type level
/// state tailored to our particular shader. This lets us not only assure that our bind
/// groups and vertex buffers are set before we can draw, but also prevents us from
/// setting these in out-of-bounds slots or indexes.
pub fn enhanced_render_pass(qty_vertex_buffers: usize, qty_bind_groups: usize) -> TokenStream {

    let vertex_buffers = (0..qty_vertex_buffers)
        .map(|idx| {
            let vb = format_ident!("VB{}", idx);
            quote!(#vb)
        })
        .collect::<Vec<_>>();
    let bind_groups = (0..qty_bind_groups)
        .map(|idx| {
            let bg = format_ident!("BG{}", idx);
            quote!(#bg)
        }).collect::<Vec<_>>();
    let mut type_params = vertex_buffers.clone();
    type_params.append(&mut bind_groups.clone());


    let needing_vertex_buffers = (0..qty_vertex_buffers)
        .map(|idx| {
            let vb = format_ident!("NeedsVertexBuffer{}", idx);
            quote!(#vb)
        })
        .collect::<Vec<_>>();
    let needing_bind_groups = (0..qty_bind_groups)
        .map(|idx| {
            let bg = format_ident!("NeedsBindGroup{}", idx);
            quote!(#bg)
        }).collect::<Vec<_>>();
    let mut type_args_needy = needing_vertex_buffers.clone();
    type_args_needy.append(&mut needing_bind_groups.clone());
    let needy_structs = type_args_needy.clone().iter().map(|ts| quote!(pub struct #ts;)).collect::<Vec<_>>();


    let vertex_setters = vertex_buffers.clone().iter().enumerate().map(|(i, type_param)| {
        let fn_name = format_ident!("set_vertex_buffer_{}", i);
        let mut type_params_after = type_params.clone();
        type_params_after[i] = quote!(Ready);
        let slot = Literal::u32_unsuffixed(i as u32);
        quote! {
            pub fn #fn_name(mut self, buffer_slice: wgpu::BufferSlice<'rp>) -> PreparedRenderPass<'rp, #(#type_params_after),*> {
                self.render_pass.set_vertex_buffer(#slot, buffer_slice);
                PreparedRenderPass {
                    render_pass: self.render_pass,
                    type_state: std::marker::PhantomData,
                }
            }
        }
    }).collect::<Vec<TokenStream>>();

    let bind_group_setters = bind_groups.clone().iter().enumerate().map(|(i, type_param)| {
        let fn_name = format_ident!("set_bind_group_{}", i);
        let mut type_params_after = type_params.clone();
        type_params_after[vertex_buffers.len() + i] = quote!(Ready);
        let bg_name = format_ident!("BindGroup{}", i);
        quote! {
            pub fn #fn_name(mut self, bind_group: &'rp bind_groups::#bg_name) -> PreparedRenderPass<'rp, #(#type_params_after),*> {
                bind_group.set(&mut self.render_pass);
                PreparedRenderPass {
                    render_pass: self.render_pass,
                    type_state: std::marker::PhantomData,
                }
            }
        }
    }).collect::<Vec<TokenStream>>();

    let type_args_all_ready = (0..type_params.len()).map(|_| quote!(Ready)).collect::<Vec<_>>();

    quote! {
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
            pub fn set<'s, 'rp>(&'s self, mut render_pass: wgpu::RenderPass<'rp>) -> PreparedRenderPass<'rp, #(#type_args_needy),*> where 's: 'rp {
                render_pass.set_pipeline(&self.0);
                PreparedRenderPass {
                    render_pass,
                    type_state: std::marker::PhantomData,
                }
            }
        }

        #(#needy_structs)*
        pub struct Ready;
        pub struct PreparedRenderPass<'rp, #(#type_params),*> {
            render_pass: wgpu::RenderPass<'rp>,
            type_state: std::marker::PhantomData<(#(#type_params),*)>,
        }
        impl<'rp, #(#type_params),*> PreparedRenderPass<'rp, #(#type_params),*> {
            pub fn inner(&mut self) -> &mut wgpu::RenderPass<'rp> {
                &mut self.render_pass
            }

            #(#vertex_setters)*

            #(#bind_group_setters)*
        }
        impl<'rp> PreparedRenderPass<'rp, #(#type_args_all_ready),*> {

            pub fn into_inner(self) -> wgpu::RenderPass<'rp> {
                self.render_pass
            }
        }
        impl<'rp> core::ops::Deref for PreparedRenderPass<'rp, #(#type_args_all_ready),*> {
            type Target = wgpu::RenderPass<'rp>;
            fn deref(&self) -> &Self::Target {
                &self.render_pass
            }
        }
        impl<'rp> core::ops::DerefMut for PreparedRenderPass<'rp, #(#type_args_all_ready),*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.render_pass
            }
        }
    }
}