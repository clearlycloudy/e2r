use interface::i_window::IWindow;
use interface::i_game_logic::IGameLogic;
use interface::i_renderer::IRenderer;

pub trait IKernel < W: IWindow,
                    G: IGameLogic< EventInput = W::EventType >,
                    R: IRenderer< EventRender = G::EventRender > >
    : AsMut< W > + AsMut< G > + AsMut< R >

//possible use trait bounds and delegate traits to subfields in concrete implementer when this is supported by Rust (https://github.com/rust-lang/rfcs/pull/1406)
    // IWindow +
    // IGameLogic< EventInput = < Self as IWindow >::EventType > +
    // IRenderer< EventRender = < Self as IGameLogic >::EventRender >
{
    fn new() -> Result< Self, & 'static str > where Self: Sized;

    fn new_with < F > ( f: F ) -> Result< Self, & 'static str >
        where F: FnOnce() -> Result< Self, & 'static str >, Self: Sized
    {
        f()
    }

    fn init_hook( & mut self ) -> Result< (), & 'static str > { Ok( () ) }

    fn deinit_hook( & mut self ) -> Result< (), & 'static str > { Ok( () ) }

    fn run( & mut self ) -> Result< (), & 'static str > {

        self.init_hook()?;
            
        info!( "kernel running." );
        
        //foever loop and process results until exit conditions are caught
        let mut running = true;

        #[allow(unused_mut)]
        let mut sigs_for_window = vec![];

        while running {

            //process windowing events into buffer
            (self.as_mut() as & W).make_current()?;

            (self.as_mut() as & mut W).per_frame_setup()?;

            (self.as_mut() as & mut W).handle_signal_request( sigs_for_window.as_slice() )?;

            let mut events_window : Vec< W::EventType > = vec![];
            match (self.as_mut() as & mut W).handle_events_pass_thru() {
                Some( x ) => {
                    events_window.push( x );
                },
                _ => {},
            }
            
            let ( events_render, signal_exit ) : ( Vec<  _ >, bool ) = (self.as_mut() as & mut G).process_input_events( & events_window[..] );

            if signal_exit {
                running = false;
            }

            (self.as_mut() as & mut R).process_render_events( & events_render[..] ).is_ok();

            (self.as_mut() as & mut W).swap_buf();
        }
        
        info!( "kernel shutdown." );

        self.deinit_hook()?;

        Ok( () )
    }
}
