use crate::renderer::RendererError;
use cocoa::{
    appkit::{
        NSApplication, NSBackingStoreType, NSEventMask, NSEventType, NSView, NSWindowStyleMask,
    },
    base::{id as cocoa_id, nil, BOOL, NO, YES},
    foundation::{NSAutoreleasePool, NSDefaultRunLoopMode, NSPoint, NSRect, NSSize, NSString},
};
use core_graphics::display::{CGDisplayBounds, CGMainDisplayID, CGRect};
use metal::{
    foreign_types::ForeignType,
    {Device, MetalLayer},
};
use objc::{class, msg_send, sel, sel_impl};

/// Represents a macOS window with Metal rendering support
#[derive(Clone)]
pub struct MacOSWindow {
    // These fields are necessary to keep the window and view alive,
    // even though they are not used after creation.
    #[allow(dead_code)]
    window: cocoa_id,
    #[allow(dead_code)]
    view: cocoa_id,
}

impl MacOSWindow {
    /// Creates a new MacOSWindow with the specified dimensions and title
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the window in points
    /// * `height` - The height of the window in points
    /// * `title` - The title of the window
    ///
    /// # Returns
    ///
    /// A Result containing the new MacOSWindow or a RendererError
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, RendererError> {
        unsafe {
            let pool = NSAutoreleasePool::new(nil);

            // Calculate the window frame
            let screen_frame = CGDisplayBounds(CGMainDisplayID());
            let window_frame = NSRect::new(
                NSPoint::new(
                    (screen_frame.size.width - width as f64) * 0.5,
                    (screen_frame.size.height - height as f64) * 0.5,
                ),
                NSSize::new(width as f64, height as f64),
            );

            // Create and configure the window
            let window: cocoa_id = msg_send![class!(NSWindow), alloc];
            let window: cocoa_id = msg_send![
                window,
                initWithContentRect:window_frame
                styleMask:NSWindowStyleMask::NSTitledWindowMask.bits() | NSWindowStyleMask::NSClosableWindowMask.bits() | NSWindowStyleMask::NSMiniaturizableWindowMask.bits() | NSWindowStyleMask::NSResizableWindowMask.bits()
                backing:NSBackingStoreType::NSBackingStoreBuffered
                defer:NO
            ];

            let device = Device::system_default().ok_or(RendererError::DeviceNotFound)?;
            let metal_layer = MetalLayer::new();
            metal_layer.set_device(&device);

            let title = NSString::alloc(nil).init_str(title);
            let () = msg_send![window, setTitle:title];
            let () = msg_send![window, makeKeyAndOrderFront:nil];

            // Create and set up the view
            let view = NSView::alloc(nil).init();
            let () = msg_send![window, setContentView:view];

            // // Set up Metal layer
            // let device = Device::system_default().expect("No Metal device found");
            // let metal_layer = MetalLayer::new();
            // metal_layer.set_device(&device);
            // metal_layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
            // metal_layer.set_presents_with_transaction(false);

            let () = msg_send![view, setWantsLayer:YES];
            let () = msg_send![view, setLayer:metal_layer.as_ptr()];

            let () = msg_send![pool, release];

            Ok(MacOSWindow { window, view })
        }
    }

    /// Runs the main event loop
    ///
    /// # Arguments
    ///
    /// * `callback` -  A function to be called on each iteration of the event loop
    ///
    /// # Returns
    ///
    /// A result indicating success or a RendererError
    pub fn run_loop<F>(&self, mut callback: F) -> Result<(), RendererError>
    where
        F: FnMut(cocoa_id) -> Result<(), Box<dyn std::error::Error>>,
    {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicyRegular);
            app.finishLaunching();

            while self.is_window_open() {
                let pool = NSAutoreleasePool::new(nil);

                // get the next event
                let event: cocoa_id = msg_send![
                    app,
                    nextEventMatchingMask:NSEventMask::NSAnyEventMask
                    untilDate:nil
                    inMode:NSDefaultRunLoopMode
                    dequeue:YES
                ];

                if event != nil {
                    let event_type: NSEventType = msg_send![event, type];
                    match event_type {
                        NSEventType::NSApplicationDefined => {
                            // Call the user-provided callback
                            if let Err(e) = callback(event) {
                                println!("Render callback error: {:?}", e);
                            }
                        }
                        _ => {
                            // Send the other events to the application
                            app.sendEvent_(event);
                        }
                    }
                } else {
                    // No events, now render
                    if let Err(e) = callback(nil) {
                        println!("Render callback error: {:?}", e);
                    }
                }

                let _: () = msg_send![pool, release];
            }
            Ok(())
        }
    }

    fn is_window_open(&self) -> bool {
        unsafe {
            let is_visible: BOOL = msg_send![self.window, isVisible];
            is_visible == YES
        }
    }

    pub fn get_view(&self) -> cocoa_id {
        self.view
    }

    pub fn get_size(&self) -> (u32, u32) {
        unsafe {
            let frame: CGRect = msg_send![self.view, frame];
            (frame.size.width as u32, frame.size.height as u32)
        }
    }
}

impl Drop for MacOSWindow {
    fn drop(&mut self) {
        unsafe {
            let () = msg_send![self.view, release];
            let () = msg_send![self.window, close];
        }
    }
}
