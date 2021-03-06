///sample implementation of game logic, also extends the game logic to _game_impl

extern crate image;
extern crate rand;
extern crate mazth;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use interface::i_ele;
use interface::i_game_logic::IGameLogic;
use interface::i_ui::{ InputFiltered, KeyCode };
// use interface::i_camera::ICamera;
use interface::i_scheduler::IScheduler;

use implement::render::renderer_gl;
use implement::render::util_gl;
use implement::render::texture;
use implement::render::camera;
use implement::render::light;
use implement::render::mesh;
use implement::render::primitive;

use self::mazth::mat;

use self::rand::Rng;
use self::image::GenericImage;

//todo: put this somewhere else
pub fn file_open( file_path: & str ) -> Option<String> {
    let path = File::open( file_path ).expect("file path open invalid");
    let mut buf_reader = BufReader::new(path);
    let mut contents = String::new();
    match buf_reader.read_to_string( & mut contents ){
        Err( e ) => { error!("{}", e ); return None },
        _ => (),
    }
    Some(contents)
}

#[derive(Copy, Clone, Debug)]
pub struct GameState {
    _exit: bool,
    _continue_compute: bool,
    _time_game: f32,
    _is_init_run_first_time: bool
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            _exit: false,
            _continue_compute: false,
            _time_game: 0.0,
            _is_init_run_first_time: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameStateChangePending {
    
}

impl Default for GameStateChangePending {
    fn default() -> GameStateChangePending {
        GameStateChangePending {
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameStateChangeApply {
    _end_compute: bool,
}

impl Default for GameStateChangeApply {
    fn default() -> GameStateChangeApply {
        GameStateChangeApply {
            _end_compute: false,
        }
    }
}

impl From< ComputeUnit > for GameStateChangeApply {
    fn from( _c: ComputeUnit ) -> Self {
        match _c {
            ComputeUnit::SignalEndCompute => {
                Self {
                    _end_compute: true
                }
            },
            _ => {
                Default::default()
            },
        }
    }
}

#[derive(Clone)]
pub enum ComputeUnit {
    SignalEndCompute,
    TBD,
}

#[derive(Clone)]
pub struct ComputeSchedule {
    _compute_units: Vec< ComputeUnit >,
    _index: usize,
}

impl IScheduler for ComputeSchedule {
    type Item = ComputeUnit;
    fn new( _items: &[Self::Item] ) -> ComputeSchedule {
        ComputeSchedule {
            _compute_units: _items.to_vec(),
            _index: 0,
        }
    }
}

impl Iterator for ComputeSchedule {
    type Item = Vec< ComputeUnit >;
    fn next( & mut self ) -> Option< Self::Item > {
        if self._index >= self._compute_units.len() {
            None
        } else {
            //todo
            let s = Some( vec![ self._compute_units[ self._index ].clone() ] );
            self._index += 1;
            s
        }
    }
}

impl From< (GameState, GameStateChangeApply) > for GameState {
    fn from( (_s, _a): (GameState, GameStateChangeApply) ) -> Self {
        //todo
        let mut s = _s.clone();
        if _a._end_compute {
            s._continue_compute = false;
        }
        s
    }
}

pub enum RenderObj {
    InitialRender { _path_shader_vs: String, _path_shader_fs: String },
    TestGeometry { _time_game: f32, _light: light::LightAdsPoint, _camera: camera::Cam },
}


impl From< RenderObj > for Vec< renderer_gl::Event > {
    fn from( _r: RenderObj ) -> Self {
        match _r {
            RenderObj::InitialRender{ _path_shader_vs, _path_shader_fs } => {
                let mut render_events = vec![];
                
                info!("game logic: first time initialization.");

                let vs_src = file_open( _path_shader_vs.as_str() ).expect("vertex shader not retrieved");
                let fs_src = file_open( _path_shader_fs.as_str() ).expect("fragment shader not retrieved");
                let event_load_shader = renderer_gl::Event::LoadShader(
                    vec![
                        ( vs_src, util_gl::ShaderType::VERTEX ),
                        ( fs_src, util_gl::ShaderType::FRAGMENT ),
                    ] );
                render_events.push( event_load_shader );

                let img = image::open( &Path::new( "core/asset/images/texture0.jpg" ) ).unwrap();
                debug!( "image dimension: {:?}", img.dimensions() );
                debug!( "image type: {:?}", img.color() );
                
                let texture0 = texture::Texture::from( &img );
                let texture_data = Vec::from( texture0 );
                let ( w, h ) = img.dimensions();
                let event_load_texture = renderer_gl::Event::LoadTexture( String::from("texture0"), texture_data, w as _, h as _ );
                render_events.push( event_load_texture );

                info!( "press q to quit." );

                render_events
            },
            RenderObj::TestGeometry{ _time_game, _light, _camera } =>{
                let mut render_events = vec![];
                
                //create some meshes for test:
                //set triangle vert positions and normals
                let mut mesh = mesh::Mesh::init( 0 );
                mesh._batch_pos.extend_from_slice( &[ -1f32, -1f32, -1f32,
                                                       5f32, -1f32, -1f32,
                                                      -1f32,  1f32, -1f32,
                                                       4f32, -1f32, 15f32,
                                                       6f32, -1f32, 15f32,
                                                       4f32,  1f32, 15f32, ] );

                mesh._batch_normal.extend_from_slice( &[ 0f32, 0f32, 1f32,
                                                         0f32, 0f32, 1f32,
                                                         0f32, 0f32, 1f32,
                                                         0f32, 0f32, 1f32,
                                                         0f32, 0f32, 1f32,
                                                         0f32, 0f32, 1f32, ] );
                
                mesh._batch_tc.extend_from_slice( &[ 0f32, 0f32,
                                                     0f32, 0f32,
                                                     0f32, 0f32,
                                                     0f32, 0f32,
                                                     0f32, 0f32,
                                                     0f32, 0f32, ] );

                let mesh_copy = mesh.clone();

                let mut mesh2 = mesh_copy.clone();
                mesh2._batch_pos.clear();
                mesh2._batch_pos.extend_from_slice( &[ -1f32+ _time_game, -1f32, -1f32,
                                                        5f32+_time_game, -1f32, -1f32,
                                                        -1f32+_time_game,  1f32, -1f32,
                                                        4f32+_time_game, -1f32, 15f32,
                                                        6f32+_time_game, -1f32, 15f32,
                                                        4f32+_time_game,  1f32, 15f32, ] );

                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( mesh2 ) ) );

                let prim_box = primitive::Poly6 { _pos: mat::Mat3x1 { _val: [ -5f32, -10f32, 5f32 ] },
                                                   _scale: mat::Mat3x1 { _val: [ 1., 1., 1. ] },
                                                   _radius: 5f32 };

                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( prim_box ) ) );

                let prim_sphere = primitive::SphereIcosahedron::init( mat::Mat3x1 { _val: [ -20f32, -10f32, 0f32 ] }, 5f32 );

                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( prim_sphere ) ) );
                
                let l = &_light;
                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( l.clone() ) ) );

                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( _camera.clone() ) ) );

                render_events
            },
        }
    }
}

pub struct GameLogic {
    //todo
    _is_init: bool,
    _lights: Vec< light::LightAdsPoint >, //could move this to be generated by game logic or within a connecting adaptor between game logic and render interface
    _cameras: Vec< camera::Cam >, //todo: replace with camera wrapper
    // _cameras_wrapper: Vec< Box< ICamera > >,
    _delta: f32, //test parameter for object velocity
    _path_shader_vs: String,
    _path_shader_fs: String,
    _state: GameState,
}

impl IGameLogic for GameLogic {

    type EventInput = InputFiltered;
    type EventRender = renderer_gl::Event;
    type GameState = GameState;
    type GameStateChangePending = GameStateChangePending;
    type GameStateChangeApply = GameStateChangeApply;
    type ComputeUnit = ComputeUnit;
    type ComputeSchedule = ComputeSchedule;
    type RenderObj = RenderObj;

    fn new() -> GameLogic {
        let mut ret = GameLogic {
            _is_init: false,
            _lights: vec![],
            _cameras: vec![],
            // _cameras_wrapper: vec![],
            _delta: 0f32,
            _path_shader_vs: String::new(),
            _path_shader_fs: String::new(),
            _state: Default::default(),
        };
        
        //lights
        let mut rng = rand::thread_rng();
        for i in 0..50 {
            let pos_x = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 6f32 - 3f32;
            let pos_y = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 6f32 - 4f32;
            let pos_z = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 6f32 + 10f32;
            let colour_r = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 1f32;
            let colour_g = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 1f32;
            let colour_b = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 1f32;
            let l = light::LightAdsPoint {
                _id: i as u64,
                _pos: mat::Mat3x1 { _val: [ pos_x, pos_y, pos_z ] },
                _ads_val_spec: mat::Mat3x1 { _val: [ colour_r, colour_g, colour_b ] },
                _ads_val_diff: mat::Mat3x1 { _val: [ colour_r, colour_g, colour_b ] },
                _ads_val_amb: mat::Mat3x1 { _val: [ colour_r, colour_g, colour_b ] },
            };
            ret._lights.push( l );
        }

        //camera
        let fov = 120f32;
        let aspect = 1f32;
        let near = 0.001f32;
        let far = 1000f32;
        let cam_foc_pos = mat::Mat3x1 { _val: [0f32, 0f32, 5f32] };
        let cam_up = mat::Mat3x1 { _val: [0f32, 1f32, 0f32] };
        let cam_pos = mat::Mat3x1 { _val: [5f32, 5f32, 20f32] };
        let cam_id = 0;
        let cam = camera::Cam::init( cam_id, fov, aspect, near, far, cam_pos, cam_foc_pos, cam_up );
        ret._cameras.push( cam );

        ret
    }

    ///do some initialization
    fn run_init_hook( & mut self ) -> Result< (), & 'static str > {
        self._path_shader_vs = String::from("core/asset/shader/ads.vs"); //some hard coded paths for now
        self._path_shader_fs = String::from("core/asset/shader/ads.fs");
        Ok( () )
    }

    ///computes changed game state given user inputs and current game state
    fn transition_states( & mut self, inputs: & [ InputFiltered ] ) -> GameStateChangePending {
        //todo

        for i in inputs.iter() {
            match i {
                &InputFiltered::Button { key: KeyCode::Q, .. } => {
                    self._state._exit = true;
                },
                _ => {},
            }
        }

        self.set_continue_compute( true );

        // state_change
        Default::default()
    }
    fn get_states( & mut self ) -> & Self::GameState {
        & self._state
    }

    fn get_states_mut( & mut self ) -> & mut Self::GameState {
        & mut self._state
    }
    fn set_continue_compute( & mut self, b: bool ) {
        self._state._continue_compute = b;
    }
    fn continue_compute( & mut self ) -> bool {
        self._state._continue_compute
    }
    fn get_computations( & mut self, _changed_game_state: & GameStateChangePending ) -> Vec< ComputeUnit > {
        //todo: transform changed game state to additional computations

        let mut _compute_units = vec![];

        //append this to signal compute cycle is complete
        _compute_units.push( ComputeUnit::SignalEndCompute );

        _compute_units
    }
    fn schedule_computes( & mut self, _computes: Vec< ComputeUnit > ) -> Vec< Self::ComputeSchedule > {
        //todo
        let mut _compute_schedule = vec![];

        _compute_schedule
    }
    fn get_renderable_components( & mut self ) -> Vec< RenderObj > {

        //todo: use game specific game logic to produce render objects instead

        // {
        //     let test_impl = self.get_game_impl();
        // }
        
        let mut v = vec![];

        if !self._state._is_init_run_first_time {
            //does this once to setup some shaders
            self._state._is_init_run_first_time = true;
            let initial_render = RenderObj::InitialRender { _path_shader_fs: self._path_shader_fs.clone(),
                                                            _path_shader_vs: self._path_shader_vs.clone() };
            v.push( initial_render );
        }

        //dummy geometry to render
        v.push( RenderObj::TestGeometry { _time_game: self._state._time_game,
                                          _light: self._lights[0].clone(),
                                          _camera: self._cameras[0].clone() } );
        
        self._state._time_game -= 0.01;

        v
    }
    fn filter_renderables( & mut self, _r: Vec< RenderObj > ) -> Vec< RenderObj > {
        //todo: add spatial accelerator algo here
        _r
    }

    fn should_exit( & mut self ) -> bool {
        self._state._exit
    }

    // fn get_game_impl( & mut self ) -> & mut GameImpl {
    //     & mut self._game_impl
    // }
}
