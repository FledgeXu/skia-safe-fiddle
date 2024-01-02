use skia_safe::{scalar, Canvas};
pub const WINDOWS_WIDTH: i32 = 800;
pub const WINDOWS_HEIGHT: i32 = 800;
#[cfg(all(target_os = "macos", feature = "metal"))]
pub fn init_window<F>(scale_factor: (scalar, scalar), draw_fn: F)
where
    F: Fn(&Canvas),
{
    use cocoa::{appkit::NSView, base::id as cocoa_id};
    use core::panic;
    use core_graphics_types::geometry::CGSize;
    use metal::{
        foreign_types::{ForeignType, ForeignTypeRef},
        Device, MTLPixelFormat, MetalLayer,
    };
    use objc::rc::autoreleasepool;
    use skia_safe::{
        gpu::{self, mtl, BackendRenderTarget, DirectContext, SurfaceOrigin},
        Color, ColorType, Size,
    };
    use winit::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        raw_window_handle::HasWindowHandle,
        window::WindowBuilder,
    };

    let size = LogicalSize::new(WINDOWS_WIDTH, WINDOWS_HEIGHT);

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = WindowBuilder::new()
        .with_inner_size(size)
        .with_title("Tester Skia")
        .build(&event_loop)
        .unwrap();
    let window_handle = window
        .window_handle()
        .expect("Failed to retrieve a window handle");
    let raw_window_handle = window_handle.as_raw();

    let device = Device::system_default().expect("Not found Device(usually a CPU).");
    let metal_layer = {
        let draw_size = window.inner_size();
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);
        unsafe {
            let view = match raw_window_handle {
                winit::raw_window_handle::RawWindowHandle::AppKit(appkit) => {
                    appkit.ns_view.as_ptr()
                }
                _ => panic!("Wrong window handle type!"),
            } as cocoa_id;
            view.setWantsLayer(true);
            view.setLayer(layer.as_ref() as *const _ as _);
        }
        layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));
        layer
    };

    let command_queue = device.new_command_queue();

    let backend = unsafe {
        mtl::BackendContext::new(
            device.as_ptr() as mtl::Handle,
            command_queue.as_ptr() as mtl::Handle,
            std::ptr::null(),
        )
    };

    let mut context = DirectContext::new_metal(&backend, None).unwrap();
    event_loop
        .run(move |event, window_target| {
            autoreleasepool(|| {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::CloseRequested => window_target.exit(),
                        WindowEvent::Resized(size) => {
                            metal_layer.set_drawable_size(CGSize::new(
                                size.width as f64,
                                size.height as f64,
                            ));
                            window.request_redraw();
                        }
                        WindowEvent::RedrawRequested => {
                            if let Some(drawable) = metal_layer.next_drawable() {
                                let drawable_size = {
                                    let size = metal_layer.drawable_size();
                                    Size::new(size.width as scalar, size.height as scalar)
                                };
                                let mut surface = unsafe {
                                    let texture_info = mtl::TextureInfo::new(
                                        drawable.texture().as_ptr() as mtl::Handle,
                                    );
                                    let backend_render_target = BackendRenderTarget::new_metal(
                                        (drawable_size.width as i32, drawable_size.height as i32),
                                        &texture_info,
                                    );
                                    gpu::surfaces::wrap_backend_render_target(
                                        &mut context,
                                        &backend_render_target,
                                        SurfaceOrigin::TopLeft,
                                        ColorType::BGRA8888,
                                        None,
                                        None,
                                    )
                                    .unwrap()
                                };

                                // Draw entry
                                surface.canvas().clear(Color::WHITE);
                                surface.canvas().scale(scale_factor);
                                draw_fn(surface.canvas());
                                context.flush_and_submit();

                                drop(surface);

                                let command_buffer = command_queue.new_command_buffer();
                                command_buffer.present_drawable(drawable);
                                command_buffer.commit();
                            }
                        }
                        _ => (),
                    }
                }
            })
        })
        .expect("run() Failed");
}

#[cfg(target_os = "windows")]
pub fn init_window() {
    println!("it's windows only");
}
