///sample implementation of kernel

use interface::i_window::IWindow;
use interface::i_game_logic::IGameLogic;
use interface::i_kernel::IKernel;
use interface::i_ui::IUi;

use implement::logic::game0::GameLogic; //example game logic to test
use implement::window::winglutin::WinGlutin;
use implement::render::renderer_gl::Renderer;
use implement::ui::input_default_glutin::XformInput;

pub struct Kernel {
    pub _windowing: WinGlutin,
    pub _input: XformInput,
    pub _game_logic: GameLogic,
    pub _renderer: Renderer,
}

///use default implementation for run method
impl IKernel< WinGlutin, XformInput, GameLogic, Renderer > for Kernel {
    fn new() -> Result< Self, & 'static str > where Self: Sized {

        info!("kernel creation." );

        let w = WinGlutin::new( 500, 500 );
        
        w.make_current()?;

        //render init need windowing to be already init
        let r = Renderer::init().expect("renderer init unsuccessful");

        let k = Kernel {
            _windowing: w,
            _input: XformInput::new(),
            _game_logic: GameLogic::new(),
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

impl AsMut< Renderer > for Kernel {
   fn as_mut( & mut self ) -> & mut Renderer {
        & mut self._renderer
    }

}
    
impl AsMut< XformInput > for Kernel {
   fn as_mut( & mut self ) -> & mut XformInput {
        & mut self._input
    }

}