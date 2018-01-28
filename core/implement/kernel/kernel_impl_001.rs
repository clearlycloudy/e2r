///sample implementation of kernel

extern crate glutin;

use interface::i_window::IWindow;
use interface::i_game_logic::IGameLogic;
// use interface::i_renderer::IRenderer;
use interface::i_kernel::IKernel;

use implement::logic::game0::GameLogic; //example game logic to test
use implement::window::winglutin::WinGlutin;
use implement::render::renderer_gl;

pub struct Kernel {
    pub _windowing: WinGlutin,
    pub _game_logic: GameLogic,
    pub _renderer: renderer_gl::Renderer, 
}

///use default implementation for run method
impl IKernel< WinGlutin, GameLogic, renderer_gl::Renderer > for Kernel {
    fn new() -> Result< Self, & 'static str > where Self: Sized {

        info!("kernel creation." );

        let w = WinGlutin::init( 500, 500 );
        
        w.make_current()?;

        let r = renderer_gl::Renderer::init().expect("renderer init unsuccessful");

        let k = Kernel {
            _windowing: w,
            _game_logic: GameLogic::init(),
            _renderer: r,
        };

        Ok( k )
    }
    fn init_hook( & mut self ) -> Result< (), & 'static str > {
        self._windowing.make_current()?;
        Ok( () )
    }
    fn deinit_hook( & mut self ) -> Result< (), & 'static str > {
        Ok( () )
    }
}

impl AsMut< WinGlutin > for Kernel {
    fn as_mut( & mut self ) -> & mut WinGlutin {
        & mut self._windowing
    }
}

impl AsMut< GameLogic > for Kernel {
    fn as_mut( & mut self ) -> & mut GameLogic {
        & mut self._game_logic
    }
}

impl AsMut< renderer_gl::Renderer > for Kernel {
   fn as_mut( & mut self ) -> & mut renderer_gl::Renderer {
        & mut self._renderer
    }

}
    
 
