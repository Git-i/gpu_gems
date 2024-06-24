use std::{collections::HashMap, sync::Arc};

use vulkano::{command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer}, device::{Device, Queue}, format::Format, image::Image};

//render graph implementation based on Granite from this blog: https://themaister.net/blog/2017/08/15/render-graphs-and-vulkan-a-deep-dive/
//I'm not doing lifetime stuff
pub struct RenderGraph
{
    passes: HashMap<String, RenderPass>,
    passes_sorted: Vec<*const RenderPass>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    textures: HashMap<String, RenderGraphTexture>,
    buffers: HashMap<String, RenderGraphBuffer>
}
pub enum RenderGraphError
{
    NonExistentPass,
    NonExistentResource,
    RedundantResource
}
pub struct RenderGraphTexture{
    size: AttachmentSize,
    format: Format,
    handle: Option<Arc<Image>>,
    external: bool
}
pub struct RenderGraphBuffer;
pub struct RenderPass
{
    pub callback: fn(AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>),
    input_images: Vec<String>,
    input_buffers: Vec<String>,
    output_images: Vec<String>,
    output_buffers: Vec<String>,
    dependencies: Vec<String>,
    graph: *mut RenderGraph
}

pub enum AttachmentSize {
    Absolute(u32, u32),
    SwapchainRelative(f32, f32)
}
impl Clone for AttachmentSize {
    fn clone(&self) -> Self {
        match self {
            Self::Absolute(arg0, arg1) => Self::Absolute(arg0.clone(), arg1.clone()),
            Self::SwapchainRelative(arg0, arg1) => Self::SwapchainRelative(arg0.clone(), arg1.clone()),
        }
    }
}

pub struct AttachmentCreateInfo
{
    size: AttachmentSize,
    format: Format,
    name: String,
}
pub enum AttachmentInfo
{
    Existing(String),
    New(AttachmentCreateInfo)
}
pub struct BufferInfo
{
    size: u64
}
impl RenderGraph
{
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> RenderGraph {
        RenderGraph {
            passes: HashMap::new(),
            device,
            queue,
            passes_sorted: Vec::new(),
            textures: HashMap::new(),
            buffers: HashMap::new()
        }
    }

    pub fn get_texture(&self, name: &String) -> Option<&RenderGraphTexture> {
        self.textures.get(name)
    }
    pub fn get_buffer(&self, name: &String) -> Option<&RenderGraphBuffer> {
        self.buffers.get(name)
    }
    pub fn texture_exists(&self, name: &String) -> bool {
        self.textures.contains_key(name)
    }
    pub fn buffers_exists(&self, name: &String) -> bool {
        self.buffers.contains_key(name)
    }

    fn create_texture(&mut self, info: AttachmentCreateInfo) -> Result<(), RenderGraphError> {
        if self.texture_exists(&info.name) {
            return Err(RenderGraphError::RedundantResource);
        }
        self.textures.insert(info.name.clone(), RenderGraphTexture::from_attachment_info(info));
        Ok(())
    }

    pub fn add_pass(&mut self, pass_name: String) -> &RenderPass {
        let pass = RenderPass::new(&mut *self);
        self.passes.insert(pass_name.clone(), pass);
        self.passes.get(&pass_name).unwrap()
    }
    fn sort_passes(&mut self) {

    }
    fn compile(&mut self) {
        self.sort_passes()
    }
    pub fn execute() {
        
    }
}
fn default_pass_callback(_: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) {}
impl RenderPass
{
    fn new(graph: *mut RenderGraph) -> RenderPass {
        RenderPass {
            callback: default_pass_callback,
            input_buffers: Vec::new(),
            input_images: Vec::new(),
            output_images: Vec::new(),
            output_buffers: Vec::new(),
            dependencies: Vec::new(),
            graph
        }
    }
    fn add_resource(vec: &mut Vec<String>, graph: &mut RenderGraph, is_texture: bool, tex_info: Option<AttachmentInfo>, buff_info: Option<BufferInfo>)
        -> Result<(), RenderGraphError> 
    {
        if is_texture == true {
            let info = tex_info.unwrap();
            match info {
                AttachmentInfo::Existing(name) => {
                    if !graph.texture_exists(&name) {
                        return Err(RenderGraphError::NonExistentResource);
                    }

                    if vec.contains(&name) {
                        Ok(())
                    } else {
                        vec.push(name);
                        Ok(())
                    }
                },
                AttachmentInfo::New(info) => {
                    let name = info.name.clone();
                    let status = graph.create_texture(info);
                    if status.is_err() {
                        status
                    } else {
                        vec.push(name);
                        Ok(())
                    }
                },
            }
        } else {
            let info = buff_info.unwrap();
            unimplemented!()
        }
    }
    pub fn add_color_input(&mut self, info: AttachmentInfo) -> Result<(), RenderGraphError> {
        let graph: &mut RenderGraph;
        unsafe{ graph = &mut *self.graph; };
        Self::add_resource(&mut self.input_images, graph, true, Some(info), None)
    }
    pub fn add_color_output(&mut self, info: AttachmentInfo) -> Result<(), RenderGraphError> {
        let graph: &mut RenderGraph;
        unsafe{ graph = &mut *self.graph; };
        Self::add_resource(&mut self.output_images, graph, true, Some(info), None)
    }
}
impl Default for AttachmentCreateInfo
{
    fn default() -> Self {
        Self { size: AttachmentSize::SwapchainRelative(1.0, 1.0), format: Default::default(), name: Default::default() }
    }
}
impl RenderGraphTexture
{
    pub fn aliasable(&self, other: &RenderGraphTexture) -> bool {
        other.format == self.format &&
        other.size == self.size &&
        !self.external && !other.external
    }
}
impl Clone for AttachmentInfo
{
    fn clone(&self) -> Self {
        match self {
            Self::Existing(arg0) => Self::Existing(arg0.clone()),
            Self::New(arg0) => Self::New(arg0.clone()),
        }
    }
}
impl Clone for AttachmentCreateInfo
{
    fn clone(&self) -> Self {
        Self { size: self.size.clone(), format: self.format.clone(), name: self.name.clone() }
    }
}
impl PartialEq for AttachmentSize{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Absolute(l0, l1), Self::Absolute(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::SwapchainRelative(l0, l1), Self::SwapchainRelative(r0, r1)) => l0 == r0 && l1 == r1,
            _ => false,
        }
    }
}
impl RenderGraphTexture
{
    fn from_attachment_info(info: AttachmentCreateInfo) -> RenderGraphTexture {
        RenderGraphTexture {
            size: info.size.clone(),
            format: info.format,
            handle: None,
            external: false,
        }
    }
}
