extern crate pretty_env_logger;

use std::any::Any;
use std::collections::HashMap;

use interface::i_renderobj;

use implement::render::renderdevice_gl;

pub trait IComponent: IComponentClone {
    fn as_any( & self ) -> & Any;
}

pub trait IComponentClone {
    fn clone_box( & self ) -> Box< IComponent >;
}

impl< T > IComponentClone for T where T: 'static + IComponent + Clone {
    fn clone_box( & self ) -> Box< IComponent > {
        Box::new( self.clone() )
    }
}

impl Clone for Box< IComponent > {
    fn clone( & self ) -> Box< IComponent > {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct ComponentRenderBuffer {
    pub _data_dict: HashMap< i_renderobj::BuffDataType, Vec<f32> >,
    // pub _render_prim_type: i_renderobj::RenderObjType,
}

impl IComponent for ComponentRenderBuffer {
    fn as_any( & self ) -> & Any {
        self
    }
}

impl ComponentRenderBuffer {
    /// # this dumps the data to render device
    pub fn flush_into_render_device( & self, rd: & mut i_renderobj::RenderDevice ) -> Result< (), & 'static str > {
        trace!("flushing into render device" );
        rd.store_buff_data( & self._data_dict )?;
        Ok( () )
    }
}

#[derive(Clone)]
pub struct ComponentRenderUniform {
    /// # stores the uniforms values
    pub _data_dict_vf: HashMap< String, Vec<f32> >,
    pub _data_dict_mat4f: HashMap< String, Vec<f32> >,
    pub _data_dict_mat3f: HashMap< String, Vec<f32> >,
    /// # this maps an id to multiple uniforms
    pub _data_uniform_group: HashMap< u64, Vec< String > >,
}

impl Default for ComponentRenderUniform {
    fn default() -> ComponentRenderUniform {
        ComponentRenderUniform {
            _data_dict_vf: HashMap::new(),
            _data_dict_mat4f: HashMap::new(),
            _data_dict_mat3f: HashMap::new(),
            _data_uniform_group: HashMap::new(),
        }
    }
}

impl IComponent for ComponentRenderUniform {
    fn as_any( & self ) -> & Any {
        self
    }
}

impl ComponentRenderUniform {
    /// # this dumps the data to uniform manager
    pub fn flush_into_uniform_collection( & self, shader_program: i64, uc: & mut renderdevice_gl::RenderUniformCollection ) -> Result< (), & 'static str > {
        trace!("flushing into uniform collection" );
        for ( ref k, ref v ) in self._data_dict_vf.iter() {
            uc.set_uniform_f( shader_program as _, (*k).as_str(), renderdevice_gl::UniformType::VEC, &v[..] );
        }
        for ( ref k, ref v ) in self._data_dict_mat4f.iter() {
            uc.set_uniform_f( shader_program as _, (*k).as_str(), renderdevice_gl::UniformType::MAT4, &v[..] );
        }
        for ( ref k, ref v ) in self._data_dict_mat3f.iter() {
            uc.set_uniform_f( shader_program as _, (*k).as_str(), renderdevice_gl::UniformType::MAT3, &v[..] );
        }
        
        for ( ref k, ref v ) in self._data_uniform_group.iter() {
            trace!("uniform group: {}, length: {}.", **k, (**v).len() );
            uc.set_group( shader_program as _, **k, (**v).clone() ).is_ok();            
        }

        Ok( () )
    }
}

/// # command for resetting draw group content
#[derive(Clone)]
pub struct ComponentDrawGroupClear {
    pub _group_id: usize,
}

impl IComponent for ComponentDrawGroupClear {
    fn as_any( & self ) -> & Any {
        self
    }
}

#[derive(Clone)]
pub struct ComponentDrawGroupDependentObjects {
    pub _group_id: usize,
    pub _obj_ids: Vec< usize >,
}

impl IComponent for ComponentDrawGroupDependentObjects {
    fn as_any( & self ) -> & Any {
        self
    }
}

#[derive(Clone)]
pub struct ComponentDrawGroupBind {
    pub _group_id: usize,
}

impl IComponent for ComponentDrawGroupBind {
    fn as_any( & self ) -> & Any {
        self
    }
}

#[derive(Clone)]
pub struct ComponentDrawGroupDependentUniforms {
    pub _group_id: usize,
    pub _uniform_ids: Vec< u64 >,
}

impl IComponent for ComponentDrawGroupDependentUniforms {
    fn as_any( & self ) -> & Any {
        self
    }
}

#[derive(Clone)]
pub struct ComponentDrawGroupDispatch {
    pub _group_id: usize,
}

impl IComponent for ComponentDrawGroupDispatch {
    fn as_any( & self ) -> & Any {
        self
    }
}

